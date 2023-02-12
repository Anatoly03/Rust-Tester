#![feature(rustc_private)]

mod repo_generator;
mod arg_reader;

// mod ast_reader;
// mod diagnostics;
// mod driver;
// mod running_code;

/**
    RUN WITH
    ```
    cargo run -- --grade example/to-grade --test example/grading
    ```
 */

fn main() {
    arg_reader::util::get_config();
}