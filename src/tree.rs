use crate::{
    constraints::Constraints,
    event::Event,
    kurbo::Size,
    piet::{Color, Piet, PietText, RenderContext},
    shell::Region,
};
use std::{any::Any, ops::Range};

mod content;
pub use content::Content;

mod entry;
pub use entry::Location;
pub(crate) use entry::{Entry, Key, Node};

pub(crate) mod mutation;

pub trait Element: AsAny {
    fn paint(&mut self, piet: &mut Piet, size: Size, content: &mut Content);

    fn layout(
        &mut self,
        constraints: &Constraints,
        content: &mut Content,
        text: &mut PietText,
    ) -> Size;

    fn event(&mut self, event: &Event, handled: &mut bool, content: &mut Content);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Default)]
pub struct Tree {
    pub(crate) content: Vec<Entry>,
}

impl Tree {
    pub fn layout(&mut self, text: &mut PietText, window_size: Size) {
        let constraints = Constraints {
            min: window_size,
            max: window_size,
        };
        let (node, tree) = self.content.split_first_mut().unwrap();
        let node = node.as_mut_node();
        let children = &node.children;
        let content = &mut Content { tree, children };
        let size = node.element.layout(&constraints, content, text);
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

    pub fn event(&mut self, event: Event) {
        let (node, tree) = self.content.split_first_mut().unwrap();
        let node = node.as_mut_node();
        let children = &node.children;
        let content = &mut Content { tree, children };
        let handled = &mut false;
        node.element.event(&event, handled, content);
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
