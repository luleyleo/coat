use shell::{
    kurbo::{Point, Size, Vec2},
    Modifiers, MouseButton, MouseButtons,
};

#[derive(Debug, Clone)]
pub enum Event {
    WindowSize(Size),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// The position of the mouse in the coordinate space of the receiver.
    pub pos: Point,
    /// The position of the mouse in the coordinate space of the window.
    pub window_pos: Point,
    /// Mouse buttons being held down during a move or after a click event.
    /// Thus it will contain the `button` that triggered a mouse-down event,
    /// and it will not contain the `button` that triggered a mouse-up event.
    pub buttons: MouseButtons,
    /// Keyboard modifiers at the time of the event.
    pub mods: Modifiers,
    /// The number of mouse clicks associated with this event. This will always
    /// be `0` for a mouse-up and mouse-move events.
    pub count: u8,
    /// Focus is `true` on macOS when the mouse-down event (or its companion mouse-up event)
    /// with `MouseButton::Left` was the event that caused the window to gain focus.
    ///
    /// This is primarily used in relation to text selection.
    /// If there is some text selected in some text widget and it receives a click
    /// with `focus` set to `true` then the widget should gain focus (i.e. start blinking a cursor)
    /// but it should not change the text selection. Text selection should only be changed
    /// when the click has `focus` set to `false`.
    pub focus: bool,
    /// The button that was pressed down in the case of mouse-down,
    /// or the button that was released in the case of mouse-up.
    /// This will always be `MouseButton::None` in the case of mouse-move.
    pub button: MouseButton,
    /// The wheel movement.
    ///
    /// The polarity is the amount to be added to the scroll position,
    /// in other words the opposite of the direction the content should
    /// move on scrolling. This polarity is consistent with the
    /// deltaX and deltaY values in a web [WheelEvent].
    ///
    /// [WheelEvent]: https://w3c.github.io/uievents/#event-type-wheel
    pub wheel_delta: Vec2,
}
impl From<&shell::MouseEvent> for MouseEvent {
    fn from(e: &shell::MouseEvent) -> Self {
        MouseEvent {
            pos: e.pos,
            window_pos: e.pos,
            buttons: e.buttons,
            mods: e.mods,
            count: e.count,
            focus: e.focus,
            button: e.button,
            wheel_delta: e.wheel_delta,
        }
    }
}
