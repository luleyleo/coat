#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub use druid::{kurbo, piet};

pub mod app;
pub mod context;
pub mod cx;
pub mod event;
pub mod id;
pub mod key;
pub mod render;
pub mod tree;
pub mod widgets;
pub mod bloom;

mod box_constraints;
pub use box_constraints::BoxConstraints;

pub trait VisualEq {
    /// Determine whether two values are the same.
    ///
    /// This is intended to always be a fast operation. If it returns
    /// `true`, the two values *must* be equal, but two equal values
    /// need not be considered the same here, as will often be the
    /// case when two copies are separately allocated.
    ///
    /// Note that "equal" above has a slightly different meaning than
    /// `PartialEq`, for example two floating point NaN values should
    /// be considered equal when they have the same bit representation.
    fn eq(&self, other: &Self) -> bool;
}
