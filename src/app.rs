use crate::{
    kurbo::Size,
    shell::{piet::Piet, Application, Region, WinHandler, WindowHandle},
    tree::Tree,
    ui::Ui,
};

pub struct App {
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
        self.tree.reconcile();
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
