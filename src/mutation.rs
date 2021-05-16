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
        for i in self.index..self.tree.content.len() {
            if matches!(self.tree.content[i], Entry::End) {
                self.tree.content.splice(self.index..i, iter::empty());
                self.index -= (self.index..i).len();
                self.index += 1;
                return;
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
