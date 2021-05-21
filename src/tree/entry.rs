use crate::{
    kurbo::{Point, Size},
    tree::Element,
};
use std::any::TypeId;

pub type Location = &'static std::panic::Location<'static>;

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
