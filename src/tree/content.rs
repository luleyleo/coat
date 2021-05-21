use crate::{
    constraints::Constraints,
    kurbo::{Point, Size},
    piet::{Piet, RenderContext},
    tree::{subtree_range, Entry, Node},
};
use shell::kurbo::Affine;
use std::ops::{Index, IndexMut};

pub struct Content<'a> {
    pub(crate) tree: &'a mut [Entry],
    pub(crate) children: &'a Vec<usize>,
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
