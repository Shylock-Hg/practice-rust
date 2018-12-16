//! # Docs
//!
//! `Docs` is the simple notes of RUST document comments.
//!


// # Documents comments
//   - Markdown syntax support

// `cargo test` will also run the examples in Documents comments.

/// Adds one to the number given.
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(docs::succ(five), 6);
/// ```
pub fn succ(x: i32) -> i32{
        x + 1
}

// # Panics, Errors and Safety;
// Panics -- Case to panic;
// Errors -- How to handle various Errors;
// Safety -- if the function is `unsafe`, why is unsafe and how avoid

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
