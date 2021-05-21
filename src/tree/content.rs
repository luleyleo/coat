use shell::kurbo::Rect;

use crate::{
    constraints::Constraints,
    event::Event,
    kurbo::{Affine, Point, Size},
    piet::{Piet, PietText, RenderContext},
    tree::{subtree_range, Entry, Node},
};
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

    pub fn layout(&mut self, constraints: &Constraints, text: &mut PietText) -> Size {
        let MutTreeNode { node, tree } = self;
        let children = &node.children;
        let content = &mut Content { tree, children };
        node.size = node.element.layout(constraints, content, text);
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

    pub fn event(&mut self, event: &Event, handled: &mut bool) {
        if !*handled {
            let MutTreeNode { node, tree } = self;
            let rect = Rect::from_origin_size(node.position, node.size);

            let recurse = match event {
                Event::MouseMove(mouse_event) if rect.contains(mouse_event.pos) => {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos -= node.position.to_vec2();
                    Some(Event::MouseMove(mouse_event))
                }
                Event::MouseDown(mouse_event) if rect.contains(mouse_event.pos) => {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos -= node.position.to_vec2();
                    Some(Event::MouseDown(mouse_event))
                }
                Event::MouseUp(mouse_event) if rect.contains(mouse_event.pos) => {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos -= node.position.to_vec2();
                    Some(Event::MouseUp(mouse_event))
                }
                _ => None,
            };

            if let Some(event) = &recurse {
                let children = &node.children;
                let content = &mut Content { tree, children };
                node.element.event(event, handled, content);
            }
        }
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
