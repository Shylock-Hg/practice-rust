fn main() {
        // naked reference
        let x = 5;
        let y = &x;  // reference to x -- &
        assert_eq!(5, x);
        assert_eq!(5, *y);  // dereference -- *

        // type with Deref trait
        let y = Box::new(5);
        assert_eq!(5, *y);  // dereference like a naked reference -- *

        // custom type implement Deref trait
        let y = MyBox::new(5);
        assert_eq!(5, *y);
}

// Implement the Deref trait to customize the dereference operator *.
struct MyBox<T>(T);

impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
                MyBox(x)
        }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {  // reference to anti moving
                &self.0
        }
}

// Deref and mutability
// 1. From &T to &U when T: Deref<Target=U>
// 2. From &mut T to &mut U when T: DerefMut<Target=U>
// 3. From &mut T to &U when T: Deref<Target=U>
