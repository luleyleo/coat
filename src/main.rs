use druid_shell::{piet::Piet, Region, WinHandler, WindowHandle};

pub use druid_shell as shell;
pub use druid_shell::kurbo;
pub use druid_shell::piet;

use shell::{
    piet::{Color, RenderContext},
    Application, WindowBuilder,
};

struct App {
    handle: WindowHandle,
}

impl App {
    pub fn new() -> Self {
        App {
            handle: WindowHandle::default(),
        }
    }
}

impl WinHandler for App {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        piet.fill(&invalid.bounding_box(), &Color::FUCHSIA);
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit();
    }
}

fn main() {
    let application = Application::new().unwrap();

    let app = App::new();
    let mut builder = WindowBuilder::new(application.clone());
    builder.set_handler(Box::new(app));
    builder.set_title("Coat");
    let window = builder.build().unwrap();
    window.show();

    application.run(None);
}
