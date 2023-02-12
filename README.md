# Ideas

Grading project with following given repositories:

## Repositories

```py
################# To-Grade Repository 
├── src
│   ├── program
│   │   └── ... # Grade program in this directory.
│   ├── main.rs
│   └── ...
├── Cargo.lock
└── Cargo.toml

################# Grading Repository
├── src
│   ├── main.rs
│   ├── ...
│   └── test
│       ├── ... # Unit Tests grading code
│       └── grading.rs
├── Cargo.lock
└── Cargo.toml

################# Generated Code Grading
├── src         #
│   ├── main.rs #
│   ├── program # To-Grade
│   │   └── ... # To-Grade
│   ├── ...     #
│   └── test    # Grading
│       └── ... # Grading
├── Cargo.lock  #
└── Cargo.toml  #
```

## Writing Tests

### To-Grade 

```rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

### Grading Tests

```rs
use crate::path::to::*;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
```

TODO use this for better testing api: https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html

### Ideas for Grading IO

IO Tests should execute a method on another thread and communicate with it over `sout` and `sin`.
Powerful behaviour to access print behaviour should also be included.

```rs
#[cfg(test)]
mod io_grading {
    use crate::{main};

    #[test, io_test(main)]
    pub fn hello_test() {
        println!("Write 0 to get greeted or write 1 to see a crab."); // Assert 
        sys_in!("0"); // Write to system.out -> system.in in grading file
        println!("Hello, World!");
    }

    #[test, io_test(main)]
    pub fn crab_test() {
        println!("Write 0 to get greeted or write 1 to see a crab.");
        sys_in!("1");
        println!("🦀");
    }
}
```