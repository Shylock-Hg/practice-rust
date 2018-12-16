fn main() {
        let x = CustomSmartPointer {data: String::from("Hello World!")};
        // drop(x);  // call drop manually
}  // call the drop

struct CustomSmartPointer {
        data: String,
}

impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data: {}!",
                        self.data);
        }
}
