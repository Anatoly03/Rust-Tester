#![feature(rustc_private)]
#![feature(try_blocks)]

use std::path::Path;

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

    let k = repo_generator::repo::Repository::try_from(Path::new("example/grading"));

    if let Ok(repo) = k {
        println!("{:?}", repo.files)
    }
}