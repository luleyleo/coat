use crate::{
    constraints::Constraints,
    context::ElementCtx,
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Handled(pub bool);
impl Handled {
    pub fn handled(self) -> bool {
        self.0
    }
}
impl From<Handled> for bool {
    fn from(h: Handled) -> Self {
        h.0
    }
}
impl From<bool> for Handled {
    fn from(b: bool) -> Self {
        Handled(b)
    }
}
impl PartialEq<bool> for Handled {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

pub trait Element: AsAny {
    fn paint(&mut self, element: &mut ElementCtx, piet: &mut Piet, content: &mut Content);

    fn layout(
        &mut self,
        element: &mut ElementCtx,
        constraints: &Constraints,
        content: &mut Content,
        text: &mut PietText,
    ) -> Size;

    fn event(&mut self, element: &mut ElementCtx, event: &Event, content: &mut Content) -> Handled;
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
    pub fn content(&mut self) -> Content {
        Content {
            tree: &mut self.content,
            children: &[0],
        }
    }

    pub fn layout(&mut self, text: &mut PietText, window_size: Size) {
        let constraints = Constraints {
            min: window_size,
            max: window_size,
        };

        let content = &mut self.content();
        let mut root = content.get_mut(0).unwrap();

        let content_size = root.layout(&constraints, text);
        assert_eq!(content_size, window_size);
    }

    pub fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        piet.fill(&invalid.bounding_box(), &Color::BLACK);

        let content = &mut self.content();
        let mut root = content.get_mut(0).unwrap();

        root.paint(piet);
    }

    pub fn event(&mut self, event: Event) {
        let content = &mut self.content();
        let mut root = content.get_mut(0).unwrap();

        let _handled = root.event(&event);
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
                Entry::Begin(child_node) if depth == 0 => {
                    depth += 1;
                    node.children.push(index);
                    Self::reconcile_subtree(&mut tree[index..]);
                    node.requests.requires_im_pass |= child_node.requests.requires_im_pass;
                    node.requests.requires_layout |= child_node.requests.requires_layout;
                    node.requests.requires_paint |= child_node.requests.requires_paint;
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
    for (i, e) in tree[index..].iter().enumerate().skip(1) {
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
