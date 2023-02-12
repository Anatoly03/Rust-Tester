#![feature(rustc_private)]

// mod ast_reader;
// mod diagnostics;
mod driver;

fn main() {
    // ast_reader::run();
    // diagnostics::run();
    driver::run();
}