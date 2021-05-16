use std::any::{Any, TypeId};
use std::iter;

use druid_shell::{piet::Piet, Region, WinHandler, WindowHandle};

pub use druid_shell as shell;
pub use druid_shell::kurbo;
pub use druid_shell::piet;

use shell::{
    kurbo::{Point, Size},
    piet::{Color, RenderContext},
    Application, WindowBuilder,
};

type Location = &'static std::panic::Location<'static>;

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

#[derive(Default)]
pub struct Tree {
    content: Vec<Entry>,
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

pub struct Ui<'a> {
    mutation: TreeMutation<'a>,
}
impl<'a> Ui<'a> {
    pub fn new(tree: &'a mut Tree) -> Self {
        let mutation = TreeMutation::new(tree);
        Ui { mutation }
    }

    pub fn add<E, C>(&mut self, location: Location, element: E, content: C)
    where
        E: Element + 'static,
        C: FnOnce(&mut Ui),
    {
        let element = Box::new(element);
        if let Some(node) = self.mutation.next(location) {
            node.element = element;
            content(self);
            self.mutation.end_existing();
        } else {
            self.mutation.insert(location, element);
            content(self);
            self.mutation.end_new();
        }
    }
}

pub trait Element: Any {
    fn paint(&mut self, piet: &mut Piet, size: Size);

    fn layout(&self, constraints: &Constraints) -> Size;
}

pub struct Constraints {
    pub min: Size,
    pub max: Size,
}

pub struct ButtonElement {
    color: Color,
}
impl Element for ButtonElement {
    fn paint(&mut self, piet: &mut Piet, size: Size) {
        piet.fill(&size.to_rect(), &self.color);
    }

    fn layout(&self, constraints: &Constraints) -> Size {
        constraints.max
    }
}

#[track_caller]
fn button(ui: &mut Ui, color: Color) {
    let location = std::panic::Location::caller();
    ui.add(location, ButtonElement { color }, |_| {});
}

struct App {
    handle: WindowHandle,
    tree: Tree,
    logic: Box<dyn FnMut(&mut Ui)>,
    size: Size,
}

impl App {
    pub fn new(logic: impl Fn(&mut Ui) + 'static) -> Self {
        App {
            handle: WindowHandle::default(),
            tree: Tree::default(),
            logic: Box::new(logic),
            size: Size::ZERO,
        }
    }

    pub fn rebuild(&mut self) {
        let mut ui = Ui::new(&mut self.tree);
        (self.logic)(&mut ui);
    }
}

impl WinHandler for App {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
        self.rebuild();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        self.tree.layout(self.size);
        self.tree.paint(piet, invalid);
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit();
    }
}

fn demo_app(ui: &mut Ui) {
    button(ui, Color::RED);
}

fn main() {
    let application = Application::new().unwrap();

    let app = App::new(demo_app);
    let mut builder = WindowBuilder::new(application.clone());
    builder.set_handler(Box::new(app));
    builder.set_title("Coat");
    let window = builder.build().unwrap();
    window.show();

    application.run(None);
}
