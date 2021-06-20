use shell::kurbo::Rect;

use crate::{
    constraints::Constraints,
    context::ElementCtx,
    event::Event,
    kurbo::{Affine, Point, Size},
    piet::{Piet, PietText, RenderContext},
    tree::{Entry, Node},
};
use std::ops::{Index, IndexMut};

use super::Handled;

pub struct Content<'a> {
    pub(crate) tree: &'a mut [Entry],
    pub(crate) children: &'a [usize],
}

pub struct MutTreeNode<'a> {
    pub(crate) node: &'a mut Node,
    tree: &'a mut [Entry],
}

impl<'a> MutTreeNode<'a> {
    pub fn set_origin(&mut self, origin: Point) {
        self.node.state.position = origin;
    }

    pub fn layout(&mut self, constraints: &Constraints, text: &mut PietText) -> Size {
        let MutTreeNode { node, tree } = self;
        node.requests.requires_layout = false;

        let children = &node.children;
        let content = &mut Content { tree, children };
        let element_ctx = &mut ElementCtx::from(&**node);

        let old_size = node.state.size;
        node.state.size = node.element.layout(element_ctx, constraints, content, text);
        node.requests.requires_paint |= old_size != node.state.size;
        element_ctx.apply_to_node(node);

        node.state.size
    }

    pub fn paint(&mut self, piet: &mut Piet) {
        piet.with_save(|piet| {
            let MutTreeNode { node, tree } = self;
            piet.transform(Affine::translate(node.state.position.to_vec2()));
            let children = &node.children;
            let content = &mut Content { tree, children };

            let element_ctx = &mut ElementCtx::from(&**node);
            node.element.paint(element_ctx, piet, content);
            element_ctx.apply_to_node(node);

            Ok(())
        })
        .unwrap();
    }

    pub fn event(&mut self, event: &Event) -> Handled {
        let MutTreeNode { node, tree } = self;

        let state = node.state;
        let rect = Rect::from_origin_size(state.position, state.size);

        let event = match event {
            Event::MouseMove(mouse_event) if rect.contains(mouse_event.pos) => {
                let mut mouse_event = mouse_event.clone();
                mouse_event.pos -= state.position.to_vec2();
                Some(Event::MouseMove(mouse_event))
            }
            Event::MouseMove(_) => None,
            Event::MouseDown(mouse_event) if rect.contains(mouse_event.pos) => {
                let mut mouse_event = mouse_event.clone();
                mouse_event.pos -= state.position.to_vec2();
                Some(Event::MouseDown(mouse_event))
            }
            Event::MouseDown(_) => None,
            Event::MouseUp(mouse_event) if rect.contains(mouse_event.pos) => {
                let mut mouse_event = mouse_event.clone();
                mouse_event.pos -= state.position.to_vec2();
                Some(Event::MouseUp(mouse_event))
            }
            Event::MouseUp(_) => None,
            _ => Some(event.clone()),
        };

        if let Some(event) = event {
            let children = &node.children;
            let content = &mut Content { tree, children };

            let element_ctx = &mut ElementCtx::from(&**node);
            let handled = node.element.event(element_ctx, &event, content);
            element_ctx.apply_to_node(node);

            handled
        } else {
            Handled(false)
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

    pub fn get_mut(&mut self, index: usize) -> Option<MutTreeNode> {
        if index >= self.children.len() {
            return None;
        }

        let child_index = self.children[index];
        let content_length = self.tree[child_index].as_node().length;

        let (head, tail) = self.tree.split_at_mut(child_index + 1);
        let node = head[child_index].as_mut_node();
        let tree = &mut tail[..content_length];

        Some(MutTreeNode { node, tree })
    }

    pub fn iter_mut(&mut self) -> ContentIterMut {
        ContentIterMut {
            tree: self.tree,
            children: self.children,
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

pub struct ContentIterMut<'a> {
    tree: &'a mut [Entry],
    children: &'a [usize],
    next: usize,
}

impl<'a> Iterator for ContentIterMut<'a> {
    type Item = MutTreeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.children.len() {
            return None;
        }
        self.next += 1;
        let content_length = self.tree[0].as_node().length;

        let (node_entry, tail_with_tree) = self.tree.split_first_mut().unwrap();
        let (tree, tail) = tail_with_tree.split_at_mut(content_length);
        let node = node_entry.as_mut_node();

        // Break lifetime dependence on 'self
        let node: &'a mut Node = unsafe { &mut *(node as *mut Node) };
        let tree: &'a mut [Entry] = unsafe { &mut *(tree as *mut [Entry]) };
        // Not sure why this one needs it as well
        let tail: &'a mut [Entry] = unsafe { &mut *(tail as *mut [Entry]) };

        self.tree = &mut tail[1..]; // exclude Entry::End from `node`
        Some(MutTreeNode { node, tree })
    }
}
