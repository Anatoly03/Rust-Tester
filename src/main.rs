#![feature(rustc_private)]

use std::process::Command;

mod repository;
mod arg_reader;
mod templates;

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
    let config = arg_reader::util::get_config();

    let grading = repository::repo::Repository::from(config.grading.unwrap().as_str()).unwrap();
    let to_grade = repository::repo::Repository::from(config.to_grade.unwrap().as_str()).unwrap();

    let generated = repository::util::combine(grading, to_grade);

    let _ = generated.write_to("debug_test");

    // TODO change to use rustc

    let echo = Command::new("cargo")
            .current_dir("debug_test")
            .args(["test", "--", "-Z", "unstable-options", "--format", "json"])
            .output()
            .expect("failed to execute process");

    let s = String::from_utf8(echo.stdout).unwrap(); //.lines();

    println!("{}", s);

    // println!("{:#?}", generated.files);

    // println!("{:#?}", to_grade.files);
}