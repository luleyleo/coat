pub extern crate druid_shell as shell;
pub use druid_shell::kurbo;
pub use druid_shell::piet;

mod app;
mod constraints;
mod demo;
mod mutation;
mod tree;
mod ui;

fn main() {
    demo::main();
}
