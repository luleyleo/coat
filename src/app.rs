use shell::piet::PietText;

use crate::{
    event::{Event, MouseEvent},
    kurbo::Size,
    shell::{piet::Piet, Application, Region, WinHandler, WindowHandle},
    tree::Tree,
    ui::Ui,
};

pub struct App {
    handle: WindowHandle,
    tree: Tree,
    logic: Box<dyn FnMut(&mut Ui)>,
    text: Option<PietText>,
    size: Size,
}

impl App {
    pub fn new(logic: impl Fn(&mut Ui) + 'static) -> Self {
        App {
            handle: WindowHandle::default(),
            tree: Tree::default(),
            logic: Box::new(logic),
            text: None,
            size: Size::ZERO,
        }
    }

    fn needs_rebuild(&mut self) -> bool {
        self.tree
            .content()
            .get_mut(0)
            .map(|tree_node| tree_node.node.requires_im_pass)
            .unwrap_or(true)
    }

    pub fn rebuild_if_needed(&mut self) {
        self.tree.reconcile();
        if self.needs_rebuild() {
            self.rebuild();
        }
    }

    pub fn rebuild(&mut self) {
        loop {
            let mut ui = Ui::new(&mut self.tree);

            (self.logic)(&mut ui);
            let can_stop = !ui.action_emitted;

            self.tree.reconcile();
            if can_stop {
                break;
            }
        }

        self.tree.layout(self.text.as_mut().unwrap(), self.size);
    }
}

impl WinHandler for App {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
        self.text = Some(handle.text());
        self.rebuild();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        self.tree.paint(piet, invalid);
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn size(&mut self, size: Size) {
        self.size = size;
        self.tree.layout(self.text.as_mut().unwrap(), self.size);
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit();
    }

    fn mouse_move(&mut self, event: &shell::MouseEvent) {
        let coat_event = Event::MouseMove(MouseEvent::from(event));
        self.tree.event(coat_event);
        self.rebuild_if_needed();
        self.handle.invalidate();
    }

    fn mouse_down(&mut self, event: &shell::MouseEvent) {
        let coat_event = Event::MouseDown(MouseEvent::from(event));
        self.tree.event(coat_event);
        self.rebuild_if_needed();
        self.handle.invalidate();
    }

    fn mouse_up(&mut self, event: &shell::MouseEvent) {
        let coat_event = Event::MouseUp(MouseEvent::from(event));
        self.tree.event(coat_event);
        self.rebuild_if_needed();
        self.handle.invalidate();
    }
}
