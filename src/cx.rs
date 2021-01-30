use crate::{context::UpdateCtx, key::Caller, render::{AnyRenderObject, RenderObject}, tree::{RenderNode, RenderState, StateNode, Tree}};
use core::panic;
use std::any::Any;

pub struct Cx<'a> {
    tree: &'a mut Tree,
    state_index: usize,
    render_index: usize,
}

impl<'a> Cx<'a> {
    pub fn new(tree: &'a mut Tree) -> Self {
        Cx {
            tree,
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

        let node_prt = &mut self.tree.states[index] as *mut StateNode;
        let node = unsafe { &mut *node_prt };
        self.state_index = index + 1;

        if let Some(state) = node.state.downcast_mut::<T>() {
            fun(self, state);
        } else {
            // TODO: Handle wrong type of state
            panic!("Wrong type of state");
        }
    }

    pub fn render_object<R>(&mut self, caller: Caller, props: R::Props) -> Option<R::Action>
    where
        R: RenderObject + Default + Any,
    {
        let mut ctx = UpdateCtx;
        let index = self.find_render_object(caller);
        if let Some(index) = index {
            for node in &mut self.tree.renders[self.state_index..index] {
                node.dead = true;
            }
            let node = &mut self.tree.renders[index];
            if let Some(object) = node.object.as_any().downcast_mut::<R>() {
                object.update(&mut ctx, props);
            } else {
                // TODO: Think of something smart
            }
            node.state
                .actions
                .pop()
                .and_then(|action| action.downcast::<R::Action>().ok().map(|action| *action))
        } else {
            let mut object = R::default();
            object.update(&mut ctx, props);
            self.insert_render_object(caller, Box::new(object));
            None
        }
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
            .insert(self.state_index, StateNode { key, state, dead });
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
            RenderNode {
                key: caller,
                object,
                children: Tree::new(),
                state: RenderState::default(),
                dead: false,
            },
        );
    }
}
