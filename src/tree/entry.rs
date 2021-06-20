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

pub struct Node {
    pub key: Key,
    pub content_length: usize,
    pub children: Vec<usize>,
    pub element: Box<dyn Element>,
    pub state: NodeState,
    pub requests: NodeRequests,
}

#[derive(Clone, Copy)]
pub struct NodeState {
    pub position: Point,
    pub size: Size,
}

#[derive(Clone, Copy)]
pub struct NodeRequests {
    pub requires_im_pass: bool,
    pub requires_layout: bool,
    pub requires_paint: bool,
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

impl Node {
    pub fn new(key: Key, element: Box<dyn Element>) -> Self {
        Node {
            key,
            content_length: 0,
            children: Vec::new(),
            element,
            state: NodeState {
                position: Point::ZERO,
                size: Size::ZERO,
            },
            requests: NodeRequests {
                requires_im_pass: false,
                requires_layout: true,
                requires_paint: true,
            },
        }
    }
}

impl NodeRequests {
    pub fn merge(&mut self, other: NodeRequests) {
        self.requires_im_pass |= other.requires_im_pass;
        self.requires_layout |= other.requires_layout;
        self.requires_paint |= other.requires_paint;
    }
}
