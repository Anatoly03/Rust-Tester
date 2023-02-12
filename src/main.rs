#![feature(rustc_private)]

mod ast_reader;

fn main() {
    ast_reader::run();
}