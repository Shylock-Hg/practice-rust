
mod common;

#[cfg(test)]
mod tests {

        use tests_organization::adder;
        #[test]
        fn testing_adder() {
                super::common::setupt();
                assert_eq!(adder(2, 2,), 4);
        }
}

// integration test is only available for library (with or without binary) crate.
