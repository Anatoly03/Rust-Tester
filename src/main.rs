#![feature(rustc_private)]

// mod ast_reader;
// mod diagnostics;
// mod driver;
mod running_code;

fn main() {
    running_code::run();
}