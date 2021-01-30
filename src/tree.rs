use std::any::Any;

use druid::{Point, Size};

use crate::{key::Caller, render::AnyRenderObject};

#[derive(Default)]
pub struct Tree {
    pub states: Vec<StateNode>,
    pub renders: Vec<RenderNode>,
}

pub struct TreeIter<'a> {
    tree: &'a mut Tree,
    state_index: usize,
    render_index: usize,
}

pub struct StateNode {
    pub key: Caller,
    pub state: Box<dyn Any>,
    pub dead: bool,
}

pub struct RenderNode {
    pub key: Caller,
    pub object: Box<dyn AnyRenderObject>,
    pub children: Tree,
    pub state: RenderState,
    pub dead: bool,
}

#[derive(Default)]
pub struct RenderState {
    pub actions: Vec<Box<dyn Any>>,
    pub origin: Point,
    pub size: Size,
}

impl Tree {
    pub fn new() -> Self {
        Tree::default()
    }
}

impl<'a> TreeIter<'a> {
    fn new(tree: &'a mut Tree) -> Self {
        TreeIter {
            tree,
            state_index: 0,
            render_index: 0,
        }
    }
}
