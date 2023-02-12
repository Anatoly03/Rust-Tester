#![feature(rustc_private)]

use std::{env, collections::VecDeque};

mod repo_generator;

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
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();

    while args.len() > 0 {
        match args.pop_front() {
            Some(k) => match k.as_str() {
                "--test" => {
                    println!("Testing with: {}", args.pop_front().unwrap())
                },
                "--grade" => {
                    println!("Grading with: {}", args.pop_front().unwrap())
                },
                _ => todo!()
            }
            None => {
                break
            }
        }
    }
}