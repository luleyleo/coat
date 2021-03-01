use crate::ui::Ui;
use std::{collections::VecDeque, panic::Location};

pub(crate) struct StoreObject<T, M> {
    state: T,
    msg: MsgQueue<M>,
}

impl<T, M> StoreObject<T, M> {
    pub(crate) fn new(state: T) -> Self {
        StoreObject {
            state,
            msg: MsgQueue {
                queue: VecDeque::new(),
            },
        }
    }
}

pub struct Store<'a, T, M> {
    pub state: &'a T,
    pub msg: &'a mut MsgQueue<M>,
}

impl<'a, T, M> Store<'a, T, M> {
    pub(crate) fn new(object: &'a mut StoreObject<T, M>) -> Self {
        Store {
            state: &object.state,
            msg: &mut object.msg,
        }
    }
}

pub struct MsgQueue<M> {
    pub(crate) queue: VecDeque<M>,
}

impl<M> MsgQueue<M> {
    pub fn push(&mut self, msg: M) {
        self.queue.push_back(msg);
    }
}

#[track_caller]
pub fn use_store<T: 'static, M: 'static>(
    ui: &mut Ui,
    init: impl FnOnce() -> T,
    mut update: impl FnMut(&mut T, M),
    content: impl FnOnce(&mut Ui, &mut Store<T, M>),
) {
    let caller = Location::caller().into();
    ui.state_node(
        caller,
        || StoreObject::new(init()),
        |ui, store: &mut StoreObject<T, M>| {
            for msg in store.msg.queue.drain(..) {
                update(&mut store.state, msg);
            }
            content(ui, &mut Store::new(store))
        },
    )
}
