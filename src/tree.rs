use crate::{
    constraints::Constraints,
    kurbo::{Point, Size},
    piet::{Color, Piet, RenderContext},
    shell::Region,
};
use std::any::{Any, TypeId};

pub type Location = &'static std::panic::Location<'static>;

pub trait Element: Any {
    fn paint(&mut self, piet: &mut Piet, size: Size);

    fn layout(&self, constraints: &Constraints) -> Size;
}

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
    pub position: Point,
    pub size: Size,
}
impl Node {
    pub fn new(key: Key, element: Box<dyn Element>) -> Self {
        Node {
            key,
            element,
            position: Point::ZERO,
            size: Size::ZERO,
        }
    }
}

#[derive(Default)]
pub struct Tree {
    pub content: Vec<Entry>,
}
impl Tree {
    pub fn layout(&mut self, window_size: Size) {
        let constraints = Constraints {
            min: window_size,
            max: window_size,
        };
        if let Some(Entry::Begin(root)) = self.content.get_mut(0) {
            root.size = root.element.layout(&constraints);
        } else {
            panic!("root element not found");
        }
    }

    pub fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        piet.fill(&invalid.bounding_box(), &Color::BLACK);

        if let Some(Entry::Begin(root)) = self.content.get_mut(0) {
            root.element.paint(piet, root.size);
        } else {
            panic!("root element not found");
        }
    }
}
