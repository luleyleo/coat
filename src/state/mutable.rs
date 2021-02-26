use crate::ui::Ui;
use std::panic::Location;

pub struct Mutable<T> {
    init: Box<dyn FnOnce() -> T>,
}

impl<T: Default + 'static> Mutable<T> {
    pub fn new() -> Self {
        Mutable {
            init: Box::new(T::default),
        }
    }
}

impl<T: 'static> Mutable<T> {
    pub fn with(init: impl FnOnce() -> T + 'static) -> Self {
        Mutable {
            init: Box::new(init),
        }
    }

    #[track_caller]
    pub fn use_in(self, cx: &mut Ui, content: impl FnOnce(&mut Ui, &mut T)) {
        let caller = Location::caller().into();
        cx.state_node(caller, self.init, content);
    }
}
