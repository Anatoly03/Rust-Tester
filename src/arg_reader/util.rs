use std::{env, collections::VecDeque};

/**
 * Get Arguments from Environment
 */
fn get_args() -> VecDeque<String> {
    env::args().collect()
}

/**
 * Get Environment Arguments without first argument
 * https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
 */
fn get_args_clean() -> VecDeque<String> {
    let mut args: VecDeque<String> = get_args();
    args.pop_front();
    args
}

/**
 * Get Configuration from Environment Arguments
 */
pub fn get_config() {
    let mut args = get_args_clean();

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