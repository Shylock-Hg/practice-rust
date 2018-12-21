pub fn add_one(x: i32) -> i32 {
        x + 1
}

#[cfg(test)]
mod tests {
        #[test]
        fn it_works() {
                assert_eq!(2 + 2, 4);
        }

        #[test]
        fn test_add_one() {
                assert_eq!(super::add_one(3), 4);
        }
}
