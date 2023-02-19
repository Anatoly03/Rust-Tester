#![feature(rustc_private)]

use std::process::Command;

mod arg_reader;
mod repository;

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

    let output = String::from_utf8(echo.stdout).unwrap();
    let mut data = output.lines().map(|i| json::parse(i).unwrap());

    // println!("{:?}", output.lines());

    let test_count = &data.next().unwrap()["test_count"].as_i32().unwrap();

    println!("{:?}", 1 .. *test_count * 2);

    for _ in 0 .. *test_count * 2 {
        let value = &data.next().unwrap();

        println!("{value}");

        if value["event"] == "started" {
            //continue;
        }

        let status = value["event"] == "ok";
        let name = value["name"].as_str().unwrap();

        println!("{name}");

        if value["event"] == "failed" {
            let stdout = value["stdout"].as_str().unwrap();
        }
    }

    let summary = &data.next().unwrap();

    // println!("{}", summary);

    // println!("{:#?}", generated.files);

    // println!("{:#?}", to_grade.files);
}
