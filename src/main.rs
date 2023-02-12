#![feature(rustc_private)]
#![feature(try_blocks)]

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
    let config = arg_reader::util::get_config();

    let grading = repo_generator::repo::Repository::from(config.grading.unwrap().as_str());
    let to_grade = repo_generator::repo::Repository::from(config.to_grade.unwrap().as_str());

    if let Ok(repo) = grading {
        println!("{:#?}", repo.files)
    }

    if let Ok(repo) = to_grade {
        println!("{:#?}", repo.files)
    }
}