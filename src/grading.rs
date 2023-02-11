#[proc_macro_attribute]
pub fn grading(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    item
}

/**
 * TODO IDEAS
 * 
 * grade(weight) - executes tests and grades them 
 * grade_after(timestamp) - only executes tests after timestamp
 * public & hidden?
 */

#[grade(2)]
fn wrapped_function() {
    // 
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }