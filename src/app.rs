use druid::{Application, WindowDesc};

pub struct App {
    name: String,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        App { name: name.into() }
    }

    pub fn run(self) {
        // simple_logger::SimpleLogger::new().init().unwrap();

        // let app = Application::new().unwrap();

        // let mut builder = WindowBuilder::new(app.clone());
        // builder.set_handler(Box::new(Window::default()));
        // builder.set_title(&self.name);

        // let window = builder.build().unwrap();
        // window.show();

        // app.run(None);
    }
}
