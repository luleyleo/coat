use shell::kurbo::Affine;

use crate::{
    constraints::Constraints,
    kurbo::{Point, Size},
    piet::{Color, Piet, RenderContext},
    shell::Region,
};
use std::{
    any::{Any, TypeId},
    ops::{Index, IndexMut, Range},
};

pub type Location = &'static std::panic::Location<'static>;

pub trait Element: AsAny {
    fn paint(&mut self, piet: &mut Piet, size: Size, content: &mut Content);

    fn layout(&self, constraints: &Constraints, content: &mut Content) -> Size;
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        &*self
    }
}

pub struct Content<'a> {
    tree: &'a mut [Entry],
    children: &'a Vec<usize>,
}

pub struct MutTreeNode<'a> {
    node: &'a mut Node,
    tree: &'a mut [Entry],
}
impl<'a> MutTreeNode<'a> {
    pub fn set_origin(&mut self, origin: Point) {
        self.node.position = origin;
    }

    pub fn layout(&mut self, constraints: &Constraints) -> Size {
        let MutTreeNode { node, tree } = self;
        let children = &node.children;
        let content = &mut Content { tree, children };
        node.size = node.element.layout(constraints, content);
        node.size
    }

    pub fn paint(&mut self, piet: &mut Piet) {
        piet.with_save(|piet| {
            let MutTreeNode { node, tree } = self;
            piet.transform(Affine::translate(node.position.to_vec2()));
            let children = &node.children;
            let content = &mut Content { tree, children };
            node.element.paint(piet, node.size, content);
            Ok(())
        })
        .unwrap();
    }
}

pub struct ContentIterMut<'a, 'c> {
    content: &'a mut Content<'c>,
    next: usize,
}
impl<'a, 'c> Iterator for ContentIterMut<'a, 'c> {
    type Item = MutTreeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.content.children.len() {
            return None;
        }

        let child_index = self.content.children[self.next];
        let child_tree_range = subtree_range(&self.content.tree, child_index);

        let node = self.content.tree[child_index].as_mut_node();
        let node = unsafe { &mut *(node as *mut Node) };

        let tree = &mut self.content.tree[child_tree_range];
        let tree = unsafe { &mut *(tree as *mut [Entry]) };

        self.next += 1;
        Some(MutTreeNode { node, tree })
    }
}

impl<'a> Content<'a> {
    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Node> {
        self.tree.get(self.children[index]).map(Entry::as_node)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.tree
            .get_mut(self.children[index])
            .map(Entry::as_mut_node)
    }

    pub fn iter_mut(&mut self) -> ContentIterMut<'_, 'a> {
        ContentIterMut {
            content: self,
            next: 0,
        }
    }
}

impl<'a> Index<usize> for Content<'a> {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        self.tree[self.children[index]].as_node()
    }
}
impl<'a> IndexMut<usize> for Content<'a> {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        self.tree[self.children[index]].as_mut_node()
    }
}

#[derive(PartialEq, Eq)]
pub struct Key {
    pub type_id: TypeId,
    pub location: Location,
}

pub enum Entry {
    Begin(Node),
    End,
}

impl Entry {
    pub fn as_node(&self) -> &Node {
        match self {
            Entry::Begin(node) => node,
            Entry::End => panic!("Called as_node on Entry::End"),
        }
    }

    pub fn as_mut_node(&mut self) -> &mut Node {
        match self {
            Entry::Begin(node) => node,
            Entry::End => panic!("Called as_mut_node on Entry::End"),
        }
    }
}

pub struct Node {
    pub key: Key,
    pub element: Box<dyn Element>,
    pub children: Vec<usize>,
    pub position: Point,
    pub size: Size,
}

impl Node {
    pub fn new(key: Key, element: Box<dyn Element>) -> Self {
        Node {
            key,
            element,
            children: Vec::new(),
            position: Point::ZERO,
            size: Size::ZERO,
        }
    }
}

#[derive(Default)]
pub struct Tree {
    pub(crate) content: Vec<Entry>,
}

impl Tree {
    pub fn layout(&mut self, window_size: Size) {
        let constraints = Constraints {
            min: window_size,
            max: window_size,
        };
        let (node, tree) = self.content.split_first_mut().unwrap();
        let node = node.as_mut_node();
        let children = &node.children;
        let content = &mut Content { tree, children };
        let size = node.element.layout(&constraints, content);
        node.size = size;
        assert_eq!(size, window_size);
    }

    pub fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        piet.fill(&invalid.bounding_box(), &Color::BLACK);

        let (node, tree) = self.content.split_first_mut().unwrap();
        let node = node.as_mut_node();
        let children = &node.children;
        let content = &mut Content { tree, children };
        node.element.paint(piet, node.size, content);
    }

    pub fn reconcile(&mut self) {
        Self::reconcile_subtree(&mut self.content);
    }

    fn reconcile_subtree(tree: &mut [Entry]) {
        if tree.is_empty() {
            return;
        }
        assert!(matches!(tree[0], Entry::Begin(_)));

        let (node, tree) = tree.split_first_mut().unwrap();
        let node = node.as_mut_node();
        node.children.clear();

        let mut depth = 0;
        let iter_tree = unsafe { &mut *(tree as *mut [Entry]) };
        for (index, entry) in iter_tree.iter_mut().enumerate() {
            match entry {
                Entry::Begin(_child_node) if depth == 0 => {
                    depth += 1;
                    node.children.push(index);
                    Self::reconcile_subtree(&mut tree[index..]);
                }
                Entry::Begin(_) => depth += 1,
                Entry::End if depth > 0 => depth -= 1,
                Entry::End => break,
            }
        }
    }
}

pub fn subtree_range(tree: &[Entry], index: usize) -> Range<usize> {
    assert!(matches!(tree[index], Entry::Begin(_)));

    let mut depth = 0;
    for (i, e) in tree[index..].iter().enumerate() {
        match e {
            Entry::Begin(_) => depth += 1,
            Entry::End if depth > 0 => depth -= 1,
            Entry::End => {
                return (index + 1)..(index + i);
            }
        }
    }
    unreachable!("subtree_range must only be called on valid trees");
}
