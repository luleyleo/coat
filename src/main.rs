pub extern crate druid_shell as shell;
pub use druid_shell::kurbo;
pub use druid_shell::piet;

pub mod app;
pub mod constraints;
pub mod demo;
pub mod elements;
pub mod mutation;
pub mod tree;
pub mod ui;

fn main() {
    demo::main();
}
