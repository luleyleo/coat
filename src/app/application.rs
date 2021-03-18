use crate::{id::ChildCounter, tree::Children, ui::Ui};

pub enum AppEvent {
    Noop,
    Quit,
}

pub struct Application {
    id: String,
}
