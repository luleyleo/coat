use std::any::Any;

use crate::{app::AppHandle, id::WindowId, piet::Piet, shell::{self, Region}};


pub(crate) struct CoatWinHandler {
    app: AppHandle,
    id: WindowId,
}

impl shell::WinHandler for CoatWinHandler {
    fn connect(&mut self, handle: &shell::WindowHandle) {
        self.app.connect(self.id, handle);
    }

    fn prepare_paint(&mut self) {
        self.app.prepare_paint(self.id);
    }

    fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        self.app.paint(self.id, piet, invalid);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
