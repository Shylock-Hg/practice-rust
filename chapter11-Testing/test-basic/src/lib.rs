#[cfg(test)]
mod tests {
        // panic
        #[test]
        fn exploration() {
                assert_eq!(2 + 2, 4);  // passed
        }

        #[test]
        fn ne() {
                assert_ne!(2 + 3, 4);  // passed
        }

        #[test]
        fn another() {
                assert!(false, "assert!({}) failed!", false);  // failed
        }

        #[test]
        fn third() {
                panic!("panic failed!");  // failed
        }

        #[test]
        #[should_panic]
        fn forth() {  // passed
                assert!(false);
        }

        // Result
        #[test]
        fn sum() -> Result<(), i32> {  // passed
                if 2 + 2 == 4 {
                        Ok(())
                } else {
                        Err(-1)
                }
        }

        #[test]
        #[ignore]
        fn sum2() -> Result<(), i32> {  // failed
                if 2 + 3 == 4 {
                        Ok(())
                } else {
                        Err(-1)
                }
        }

/*
        #[test]
        #[should_panic]  // error usage -- only work for panic
        fn sum3() -> Result<(), i32> {  // error usage
                if 2 + 3 == 4 {
                        Ok(())
                } else {
                        Err(-1);
                }
        }
*/
}
