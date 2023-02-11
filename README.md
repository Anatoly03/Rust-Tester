# Ideas

Test project with given grading tests

```
├── src
│   ├── project-src
│   │   ├── main.rs
│   └── test-src
│       ├── grading.rs
├── Cargo.lock
└── Cargo.toml
```

## Main

```rs
pub fn add_two_numbers(a : i64, b : i64) : i64 {
    a + b
}

pub fn subtract_two_numbers(a : i64, b : i64) : i64 {
    a - b
}
```

## Grading Tests

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