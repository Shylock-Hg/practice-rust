fn main() {
        // unwinding stack or aborting by system
        // in Cargo.toml
        // [profile.release]
        // panic = "abort"

        // panic!("I'm dead!");

        let v = vec![1, 2, 3];
        // v[33];  // panic
}
