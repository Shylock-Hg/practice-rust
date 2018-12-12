pub fn adder(a: i32, b: i32) -> i32 {
        internel_adder(a, b)
}

fn internel_adder(a: i32, b: i32) -> i32 {
        a+b
}


#[cfg(test)]
mod tests {

        #[test]
        fn it_works() {
                assert_eq!(2 + 2, 4);
        }

        use super::internel_adder;

        #[test]
        fn test_adder() {
                assert_eq!(internel_adder(2, 2), 4);
        }
}

