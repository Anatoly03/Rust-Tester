# Ideas

Grading project with following given repositories:

```py
# To-Grade Repository 
├── src
│   ├── main.rs
│   └── ...
├── Cargo.lock
└── Cargo.toml

# Grading Repository
├── src
│   ├── main.rs
│   ├── ...
│   └── test
│       ├── ... # Unit Tests grading code
│       └── grading.rs
├── Cargo.lock
└── Cargo.toml
```

merges into the following code base:

```py
# Generated Code Grading
├── src
│   ├── main.rs # To-Grade
│   ├── ...     # To-Grade
│   └── test    # Grading
│       └── ... # Grading
├── Cargo.lock  # To-Grade or Grading?
└── Cargo.toml  # To-Grade or Grading?
```

### Main

```rs
pub fn add_two_numbers(a : i64, b : i64) : i64 {
    a + b
}

pub fn subtract_two_numbers(a : i64, b : i64) : i64 {
    a - b
}
```

### Grading Tests

You'll need this later: https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html

```rs
#[cfg(test)]
mod tests {
    let static DEADLINE = /* ... */;
    use crate::{add_two_numbers, subtract_two_numbers};

    #[test]
    pub fn add_test() {
        assert_eq!(15, add_two_numbers(6, 9));
    }

    #[test, test_after(DEADLINE)]
    pub fn sub_test() {
        assert_eq!(-2, sub_two_numbers(7, 9));
    }
}
```