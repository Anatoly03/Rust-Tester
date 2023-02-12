#![feature(rustc_private)]

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
    // println!("{:#?}", generated.files);

    // println!("{:#?}", to_grade.files);
}