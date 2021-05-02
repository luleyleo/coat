use std::any::Any;

use shell::{IdleToken, MouseEvent};

use crate::{
    app::AppHandle,
    event::{Event, InternalEvent},
    id::WindowId,
    kurbo::Size,
    piet::Piet,
    shell::{self, Application, KeyEvent, Region, TimerToken},
};

pub(crate) const RUN_COMMANDS_TOKEN: IdleToken = IdleToken::new(1);

/// A token we are called back with if an external event was submitted.
pub(crate) const EXT_EVENT_IDLE_TOKEN: IdleToken = IdleToken::new(2);

pub(crate) struct CoatWinHandler {
    app_state: AppHandle,
    window_id: WindowId,
}

impl CoatWinHandler {
    pub fn new(app_state: AppHandle, window_id: WindowId) -> Self {
        CoatWinHandler {
            app_state,
            window_id,
        }
    }
}

impl shell::WinHandler for CoatWinHandler {
    fn connect(&mut self, handle: &shell::WindowHandle) {
        self.app_state.connect(handle);

        let event = Event::WindowConnected;
        self.app_state.do_window_event(event, self.window_id);
    }

    fn prepare_paint(&mut self) {
        self.app_state.prepare_paint();
    }

    fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        self.app_state.paint(piet, invalid);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn size(&mut self, size: Size) {
        let event = Event::WindowSize(size);
        self.app_state.do_window_event(event, self.window_id);
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        // TODO: double-click detection (or is this done in druid-shell?)
        let event = Event::MouseDown(event.clone().into());
        self.app_state.do_window_event(event, self.window_id);
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        let event = Event::MouseUp(event.clone().into());
        self.app_state.do_window_event(event, self.window_id);
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        let event = Event::MouseMove(event.clone().into());
        self.app_state.do_window_event(event, self.window_id);
    }

    fn mouse_leave(&mut self) {
        self.app_state
            .do_window_event(Event::Internal(InternalEvent::MouseLeave), self.window_id);
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        self.app_state
            .do_window_event(Event::KeyDown(event), self.window_id)
            .is_handled()
    }

    fn key_up(&mut self, event: KeyEvent) {
        self.app_state
            .do_window_event(Event::KeyUp(event), self.window_id);
    }

    fn wheel(&mut self, event: &MouseEvent) {
        self.app_state
            .do_window_event(Event::Wheel(event.clone().into()), self.window_id);
    }

    fn zoom(&mut self, delta: f64) {
        let event = Event::Zoom(delta);
        self.app_state.do_window_event(event, self.window_id);
    }

    fn timer(&mut self, token: TimerToken) {
        self.app_state
            .do_window_event(Event::Timer(token), self.window_id);
    }

    fn request_close(&mut self) {
        self.app_state
            .do_window_event(Event::WindowCloseRequested, self.window_id);
    }

    fn destroy(&mut self) {
        Application::global().quit()
    }
}
