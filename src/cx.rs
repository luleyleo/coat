use crate::{
    context::{ContextState, UpdateCtx},
    id::ChildCounter,
    key::Caller,
    render::{AnyRenderObject, RenderObject},
    tree::{Child, ChildState, Children, State},
};
use core::panic;
use std::any::{type_name, Any};

pub struct Cx<'a> {
    tree: &'a mut Children,
    state: &'a mut ContextState<'a>,
    child_counter: &'a mut ChildCounter,
    state_index: usize,
    render_index: usize,
}

impl<'a> Cx<'a> {
    pub(crate) fn new(
        tree: &'a mut Children,
        state: &'a mut ContextState<'a>,
        child_counter: &'a mut ChildCounter,
    ) -> Self {
        Cx {
            tree,
            state,
            child_counter,
            state_index: 0,
            render_index: 0,
        }
    }

    pub fn state_node<T, I, N>(&mut self, caller: Caller, init: I, fun: N)
    where
        T: Any,
        I: FnOnce() -> T,
        N: FnOnce(&mut Cx<'a>, &mut T),
    {
        let index = self.find_state_node(caller);
        if index.is_none() {
            self.insert_state_node(caller, Box::new(init()));
        }
        let index = index.unwrap_or(self.state_index);

        for node in &mut self.tree.states[self.state_index..index] {
            node.dead = true;
        }

        let node_prt = &mut self.tree.states[index] as *mut State;
        let node = unsafe { &mut *node_prt };
        self.state_index = index + 1;

        if let Some(state) = node.state.downcast_mut::<T>() {
            fun(self, state);
        } else {
            // TODO: Handle wrong type of state
            panic!(
                "Wrong type of state. Expected {}",
                std::any::type_name::<T>()
            );
        }
    }

    pub fn render_object<R>(&mut self, caller: Caller, props: R::Props) -> Option<R::Action>
    where
        R: RenderObject + Default + Any,
    {
        let index = match self.find_render_object(caller) {
            Some(index) => index,
            None => {
                self.insert_render_object(caller, Box::new(R::default()));
                self.find_render_object(caller).unwrap()
            }
        };
        for node in &mut self.tree.renders[self.render_index..index] {
            node.dead = true;
        }
        let node = &mut self.tree.renders[index];
        if let Some(object) = node.object.as_any().downcast_mut::<R>() {
            let mut ctx = UpdateCtx {
                state: self.state,
                child_state: &mut node.state,
            };
            object.update(&mut ctx, props);
        } else {
            // TODO: Think of something smart
            panic!("Wrong node type. Expected {}", std::any::type_name::<R>())
        }
        self.render_index = index + 1;
        node.state
            .actions
            .pop()
            .and_then(|action| action.downcast::<R::Action>().ok().map(|action| *action))
    }
}

impl<'a> Cx<'a> {
    fn find_state_node(&mut self, caller: Caller) -> Option<usize> {
        let mut ix = self.state_index;
        for node in &mut self.tree.states[ix..] {
            if node.key == caller {
                return Some(ix);
            }
            ix += 1;
        }
        None
    }

    fn insert_state_node(&mut self, caller: Caller, state: Box<dyn Any>) {
        let key = caller;
        let dead = false;
        self.tree
            .states
            .insert(self.state_index, State { key, state, dead });
    }

    fn find_render_object(&mut self, caller: Caller) -> Option<usize> {
        let mut ix = self.render_index;
        for node in &mut self.tree.renders[ix..] {
            if node.key == caller {
                return Some(ix);
            }
            ix += 1;
        }
        None
    }

    fn insert_render_object(&mut self, caller: Caller, object: Box<dyn AnyRenderObject>) {
        self.tree.renders.insert(
            self.render_index,
            Child {
                key: caller,
                object,
                children: Children::new(),
                state: ChildState::new(self.child_counter.generate_id(), None),
                dead: false,
            },
        );
    }
}
