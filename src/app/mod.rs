#![allow(dead_code, unused_variables)]

use crate::{
    event::Event,
    id::{ChildCounter, WindowId},
    piet::Piet,
    shell::{self, Region},
    tree::Children,
    ui::Ui,
};
use std::{cell::RefCell, rc::Rc};

mod application;
mod win_handler;
mod window;

#[derive(Clone)]
pub(crate) struct AppHandle {
    inner: Rc<RefCell<AppHandleInner>>,
}

struct AppHandleInner {
    app: Box<dyn FnMut(&mut Ui)>,
    shell_app: shell::Application,
    child_counter: ChildCounter,
    children: Children,
}

impl AppHandle {
    pub fn initialize(&self) {
        todo!()
    }

    pub fn connect(&self, window: WindowId, handle: &shell::WindowHandle) {
        todo!()
    }

    pub fn prepare_paint(&self, window: WindowId) {
        todo!()
    }

    pub fn paint(&self, window: WindowId, piet: &mut Piet, invalid: &Region) {
        todo!()
    }

    pub fn event(&self, window: WindowId, event: Event) {
        todo!()
    }
}

pub fn run(app: impl FnMut(&mut Ui) + 'static) -> Result<(), shell::Error> {
    let application = shell::Application::new()?;

    let inner_handle = AppHandleInner {
        app: Box::new(app),
        shell_app: application.clone(),
        child_counter: ChildCounter::new(),
        children: Children::new(),
    };
    let handle = AppHandle {
        inner: Rc::new(RefCell::new(inner_handle)),
    };

    handle.initialize();

    application.run(None);

    Ok(())
}
