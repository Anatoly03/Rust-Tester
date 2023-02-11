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
#[grade("should add numbers", 50%, 75%)]
mod add_tests {
    use crate::add_two_numbers;

    #[test]
    pub fn zeroes() {
        assert_eq!(0, add_two_numbers(0, 0));
    }

    #[test]
    pub fn left_zero() {
        assert_eq!(5, add_two_numbers(0, 5));
    }

    #[test]
    pub fn right_zero() {
        assert_eq!(7, add_two_numbers(7, 0));
    }

    #[test]
    pub fn negative() {
        assert_eq!(-1, add_two_numbers(-7, 6));
    }
}

#[grade("should subtract numbers"), visible_after(/* TIMESTAMP */)]
mod sub_tests {
    use crate::subtract_two_numbers;

    #[test]
    pub fn zeroes() {
        assert_eq!(0, subtract_two_numbers(0, 0));
    }

    #[test]
    pub fn negative() {
        assert_eq!(-15, add_two_numbers(-7, 6));
    }
}
```

## Should Result

```
Test Case "should add numbers" passed
Hidden Tests will be executed after deadline
```