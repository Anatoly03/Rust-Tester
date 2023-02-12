use std::{collections::VecDeque, env};

pub struct EnvArgsConfig {
    to_grade: Option<String>,
    grading: Option<String>,
}

impl EnvArgsConfig {
    fn new() -> Self {
        Self {
            to_grade: None,
            grading: None,
        }
    }
}

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
pub fn get_config() -> EnvArgsConfig {
    let mut env_args = EnvArgsConfig::new();
    let mut args = get_args_clean();

    while args.len() > 0 {
        match args.pop_front() {
            Some(k) => match k.as_str() {
                "--test" => {
                    env_args.to_grade = Some(args.pop_front().unwrap());
                    // TODO: err handler
                }
                "--grade" => {
                    env_args.grading = Some(args.pop_front().unwrap());
                    // TODO: err handler
                }
                _ => todo!(),
            },
            None => break,
        }
    }

    env_args
}
