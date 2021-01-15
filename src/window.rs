use std::any::Any;

use crate::shell::kurbo::{Line, Size};
use crate::shell::piet::{self, Color, RenderContext};

use crate::shell::{
    Application, Cursor, FileDialogOptions, FileDialogToken, FileInfo, FileSpec, KeyEvent,
    MouseEvent, Region, TimerToken, WinHandler, WindowHandle,
};

const BG_COLOR: Color = Color::rgb8(0x27, 0x28, 0x22);
const FG_COLOR: Color = Color::rgb8(0xf0, 0xf0, 0xea);

#[derive(Default)]
pub struct Window {
    size: Size,
    handle: WindowHandle,
}

impl WinHandler for Window {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut piet::Piet, _: &Region) {
        let rect = self.size.to_rect();
        piet.fill(rect, &BG_COLOR);
        piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
    }

    fn command(&mut self, id: u32) {
        match id {
            0x100 => {
                self.handle.close();
                Application::global().quit()
            }
            0x101 => {
                let options = FileDialogOptions::new().show_hidden().allowed_types(vec![
                    FileSpec::new("Rust Files", &["rs", "toml"]),
                    FileSpec::TEXT,
                    FileSpec::JPG,
                ]);
                self.handle.open_file(options);
            }
            _ => println!("unexpected id {}", id),
        }
    }

    fn open_file(&mut self, _token: FileDialogToken, file_info: Option<FileInfo>) {
        println!("open file result: {:?}", file_info);
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        println!("keydown: {:?}", event);
        false
    }

    fn key_up(&mut self, event: KeyEvent) {
        println!("keyup: {:?}", event);
    }

    fn wheel(&mut self, event: &MouseEvent) {
        println!("mouse_wheel {:?}", event);
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        self.handle.set_cursor(&Cursor::Arrow);
        println!("mouse_move {:?}", event);
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        println!("mouse_down {:?}", event);
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        println!("mouse_up {:?}", event);
    }

    fn timer(&mut self, id: TimerToken) {
        println!("timer fired: {:?}", id);
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn got_focus(&mut self) {
        println!("Got focus");
    }

    fn lost_focus(&mut self) {
        println!("Lost focus");
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
