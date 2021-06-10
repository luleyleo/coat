use shell::WindowHandle;
use std::ops::Deref;

use super::Ctx;

pub struct WindowCtx {
    handle: WindowHandle,
}

impl Ctx for WindowCtx {
    type Message = ();

    fn msg(&mut self, _msg: Self::Message) {}
}

impl Deref for WindowCtx {
    type Target = WindowHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
