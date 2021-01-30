use crate::id::{ObjectId, WindowId};
use druid::{WindowHandle, piet::PietText};


pub struct ContextState<'a> {
    pub window_id: WindowId,
    pub window: &'a WindowHandle,
    pub text: PietText,
    pub focus_widget: Option<ObjectId>,
}

pub struct UpdateCtx;

pub struct EventCtx;

pub struct LifeCycleCtx;

pub struct LayoutCtx;

pub struct PaintCtx;
