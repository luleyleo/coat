#![allow(dead_code, unused_variables)]

use crate::{
    app::win_handler::CoatWinHandler,
    id::WindowId,
    shell::{self},
    ui::Ui,
};

mod handle;
pub(crate) mod win_handler;
mod window;

use handle::*;

pub struct App {
    title: String,
}

impl App {
    pub fn new(title: impl Into<String>) -> Self {
        App {
            title: title.into(),
        }
    }

    pub fn run(self, app: impl FnMut(&mut Ui) + 'static) -> Result<(), shell::Error> {
        let application = shell::Application::new()?;

        let handle = AppHandle::new(app, application.clone());

        handle.initialize();

        let mut builder = shell::WindowBuilder::new(application.clone());
        builder.set_handler(Box::new(CoatWinHandler::new(handle, WindowId::illigal())));
        builder.set_title(self.title);
        builder.build()?;

        application.run(None);

        Ok(())
    }
}
