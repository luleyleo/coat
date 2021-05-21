use crate::tree::{Element, Entry, Key, Location, Node, Tree};
use std::{any::Any, iter};

pub struct TreeMutation<'a> {
    tree: &'a mut Tree,
    index: usize,
}

impl<'a> TreeMutation<'a> {
    pub fn new(tree: &'a mut Tree) -> Self {
        TreeMutation { tree, index: 0 }
    }

    pub fn next(&mut self, location: Location) -> Option<&mut Node> {
        if let Some(index) = self.find(location) {
            self.tree.content.splice(self.index..index, iter::empty());
            self.index += 1;
            Some(self.tree.content[self.index - 1].as_mut_node())
        } else {
            None
        }
    }

    pub fn insert(&mut self, location: Location, element: Box<dyn Element>) -> &mut Node {
        let key = Key {
            location,
            type_id: element.type_id(),
        };
        self.tree
            .content
            .insert(self.index, Entry::Begin(Node::new(key, element)));
        self.index += 1;
        self.tree.content[self.index - 1].as_mut_node()
    }

    pub fn end_existing(&mut self) {
        let mut depth = 0;
        for i in self.index..self.tree.content.len() {
            match self.tree.content[i] {
                Entry::Begin(_) => depth += 1,
                Entry::End if depth > 0 => depth -= 1,
                Entry::End => {
                    self.tree.content.splice(self.index..i, iter::empty());
                    self.index += 1;
                    return;
                }
            }
        }
        unreachable!("end_existing called but there was no end");
    }

    pub fn end_new(&mut self) {
        self.tree.content.insert(self.index, Entry::End);
        self.index += 1;
    }

    fn find(&self, location: Location) -> Option<usize> {
        let mut depth = 0;
        for i in self.index..self.tree.content.len() {
            match &self.tree.content[i] {
                Entry::Begin(node) if depth == 0 => {
                    if node.key.location == location {
                        return Some(i);
                    } else {
                        depth += 1;
                    }
                }
                Entry::Begin(_node) => {
                    depth += 1;
                }
                Entry::End if depth == 0 => {
                    return None;
                }
                Entry::End => {
                    depth -= 1;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piet::Color;

    mod button {
        use shell::piet::RenderContext;

        use crate::{
            constraints::Constraints,
            kurbo::Size,
            piet::{Color, Piet, PietText},
            tree::{Content, Element},
        };

        pub struct ButtonElement {
            pub color: Color,
        }

        impl Default for ButtonElement {
            fn default() -> Self {
                ButtonElement {
                    color: Color::WHITE,
                }
            }
        }

        impl ButtonElement {
            pub fn new(color: Color) -> Self {
                ButtonElement { color }
            }
        }

        impl Element for ButtonElement {
            fn paint(&mut self, piet: &mut Piet, size: Size, _content: &mut Content) {
                piet.fill(&size.to_rect(), &self.color);
            }

            fn layout(
                &mut self,
                constraints: &Constraints,
                _: &mut Content,
                _: &mut PietText,
            ) -> Size {
                constraints.max
            }
        }
    }

    #[test]
    fn single_insert() {
        let mut tree = Tree::default();
        assert_eq!(tree.content.len(), 0);

        let mut mutation = TreeMutation::new(&mut tree);

        let loc1 = std::panic::Location::caller();
        let elm1 = Box::new(button::ButtonElement::new(Color::RED));

        mutation.insert(loc1, elm1);
        mutation.end_new();

        let mut iter = tree.content.iter();
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(iter.next().is_none());
    }

    #[test]
    fn double_insert() {
        let mut tree = Tree::default();
        assert_eq!(tree.content.len(), 0);

        let mut mutation = TreeMutation::new(&mut tree);

        let loc1 = std::panic::Location::caller();
        let elm1 = Box::new(button::ButtonElement::new(Color::RED));
        let loc2 = std::panic::Location::caller();
        let elm2 = Box::new(button::ButtonElement::new(Color::BLUE));

        mutation.insert(loc1, elm1);
        mutation.end_new();
        mutation.insert(loc2, elm2);
        mutation.end_new();

        let mut iter = tree.content.iter();
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(iter.next().is_none());

        assert_eq!(
            tree.content[2]
                .as_mut_node()
                .element
                .as_any()
                .downcast_ref::<button::ButtonElement>()
                .unwrap()
                .color,
            crate::piet::Color::BLUE
        )
    }

    #[test]
    fn insert_single_child() {
        let mut tree = Tree::default();
        assert_eq!(tree.content.len(), 0);

        let mut mutation = TreeMutation::new(&mut tree);

        let loc1 = std::panic::Location::caller();
        let elm1 = Box::new(button::ButtonElement::new(Color::RED));

        let loc2 = std::panic::Location::caller();
        let elm2 = Box::new(button::ButtonElement::new(Color::BLUE));

        mutation.insert(loc1, elm1);
        mutation.insert(loc2, elm2);
        mutation.end_new();
        mutation.end_new();

        let mut iter = tree.content.iter();
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(iter.next().is_none());

        assert_eq!(
            tree.content[1]
                .as_mut_node()
                .element
                .as_any()
                .downcast_ref::<button::ButtonElement>()
                .unwrap()
                .color,
            crate::piet::Color::BLUE
        )
    }

    #[test]
    fn update_single_child() {
        let mut tree = Tree::default();
        assert_eq!(tree.content.len(), 0);

        let loc1 = std::panic::Location::caller();
        let elm1 = Box::new(button::ButtonElement::new(Color::RED));

        let loc2 = std::panic::Location::caller();
        let elm2 = Box::new(button::ButtonElement::new(Color::BLUE));
        let elm2x = Box::new(button::ButtonElement::new(Color::GREEN));

        {
            let mut mutation = TreeMutation::new(&mut tree);
            mutation.insert(loc1, elm1);
            mutation.insert(loc2, elm2);
            mutation.end_new();
            mutation.end_new();
        }
        assert_eq!(tree.content.len(), 4);
        {
            let mut mutation = TreeMutation::new(&mut tree);
            assert!(mutation.next(loc1).is_some());
            let n2 = mutation.next(loc2);
            assert!(n2.is_some());
            let n2 = n2.unwrap();
            n2.element = elm2x;
            mutation.end_existing();
            mutation.end_existing();
        }

        let mut iter = tree.content.iter();
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(iter.next().is_none());

        assert_eq!(
            tree.content[1]
                .as_mut_node()
                .element
                .as_any()
                .downcast_ref::<button::ButtonElement>()
                .unwrap()
                .color,
            crate::piet::Color::GREEN
        )
    }

    #[test]
    fn remove_single_child() {
        let mut tree = Tree::default();
        assert_eq!(tree.content.len(), 0);

        let loc1 = std::panic::Location::caller();
        let elm1 = Box::new(button::ButtonElement::new(Color::RED));

        let loc2 = std::panic::Location::caller();
        let elm2 = Box::new(button::ButtonElement::new(Color::BLUE));

        {
            let mut mutation = TreeMutation::new(&mut tree);
            mutation.insert(loc1, elm1);
            mutation.insert(loc2, elm2);
            mutation.end_new();
            mutation.end_new();
        }
        assert_eq!(tree.content.len(), 4);
        {
            let mut mutation = TreeMutation::new(&mut tree);
            assert!(mutation.next(loc1).is_some());
            mutation.end_existing();
        }

        let mut iter = tree.content.iter();
        assert!(matches!(iter.next(), Some(&Entry::Begin(_))));
        assert!(matches!(iter.next(), Some(&Entry::End)));
        assert!(iter.next().is_none());

        assert_eq!(
            tree.content[0]
                .as_mut_node()
                .element
                .as_any()
                .downcast_ref::<button::ButtonElement>()
                .unwrap()
                .color,
            crate::piet::Color::RED
        )
    }
}
