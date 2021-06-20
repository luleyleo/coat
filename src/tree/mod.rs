use crate::{
    constraints::Constraints,
    event::Event,
    kurbo::Size,
    piet::{Color, Piet, PietText, RenderContext},
    shell::Region,
};

mod content;
pub use content::Content;

mod entry;
pub use entry::Location;
pub(crate) use entry::{Entry, Key, Node};

mod handled;
pub use handled::Handled;

mod element;
pub use element::Element;

mod mutation;
pub(crate) use mutation::TreeMutation;

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
                Entry::End => {
                    node.length = index;
                    break;
                }
            }
        }
    }
}
