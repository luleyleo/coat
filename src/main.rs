use std::any::TypeId;

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

pub struct Node {
    pub key: Key,
    pub element: Box<dyn Element>,
    pub position: Point,
    pub size: Size,
}
impl Node {
    pub fn new<E: Element + 'static>(key: Key, element: E) -> Self {
        Node {
            key,
            element: Box::new(element),
            position: Point::ZERO,
            size: Size::ZERO,
        }
    }
}

pub struct Ui<'a> {
    tree: &'a mut Vec<Node>,
    index: usize,
}
impl<'a> Ui<'a> {
    pub fn new(tree: &'a mut Vec<Node>) -> Self {
        Ui { tree, index: 0 }
    }

    pub fn add<E: Element + 'static>(&mut self, location: Location, element: E) {
        let key = Key {
            location,
            type_id: TypeId::of::<E>(),
        };
        for i in self.index..self.tree.len() {
            // update existing element
            if self.tree[i].key == key {
                self.tree[i].element = Box::new(element);
                self.index = i + 1;
                return;
            }
        }
        self.tree.insert(self.index, Node::new(key, element));
        self.index += 1;
    }
}

pub trait Element {
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
    ui.add(location, ButtonElement { color });
}

struct App {
    handle: WindowHandle,
    tree: Vec<Node>,
    logic: Box<dyn FnMut(&mut Ui)>,
    size: Size,
}

impl App {
    pub fn new(logic: impl Fn(&mut Ui) + 'static) -> Self {
        App {
            handle: WindowHandle::default(),
            tree: Vec::new(),
            logic: Box::new(logic),
            size: Size::ZERO,
        }
    }

    pub fn rebuild(&mut self) {
        let mut ui = Ui::new(&mut self.tree);
        (self.logic)(&mut ui);
    }

    pub fn layout(&mut self) {
        let constraints = Constraints {
            min: self.size,
            max: self.size,
        };
        for node in self.tree.iter_mut() {
            let size = node.element.layout(&constraints);
            node.size = size;
        }
    }

    pub fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        piet.fill(&invalid.bounding_box(), &Color::BLACK);

        for node in self.tree.iter_mut() {
            node.element.paint(piet, node.size);
        }
    }
}

impl WinHandler for App {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
        self.rebuild();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        self.layout();
        self.paint(piet, invalid);
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
