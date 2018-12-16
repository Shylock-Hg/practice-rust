// Commonly, reference borrow the value, and pointer hold the ownership of value.

// Pointers in standard library.
// Box<T> for allocating values in the heap.
// Rc<T>, a reference counting type that enables multiple ownership
// "Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time.

// Box<T> common situation:
// 1. A type whose size not known at compile time and must to use the type in a context that requires an exact size.
// 2. Tansfer ownership of large data but ensure the data won't be copied.
// 3. When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type.

fn main() {
        // create a i32 value in heap
        // define a variable b typed Box<i32> pointed to the value 5 in heap
        let b = Box::new(5);
        println!("b is {}!", b);

        // List usage
        let l = List::Cons(1,
                Box::new(List::Cons(2,
                        Box::new(List::Cons(3,
                                Box::new(List::Nil))))));
}  // deallocte b and the value in heap
// deallocte l
// beacase the Drop trait of Box<T>

// recursive type
/*
enum List<T> {
        Cons(T, List<T>),  // recursive definition
        Nil,
}
*/
enum List<T> {
        Cons(T, Box<List<T>>),  // not recursive definition
        Nil,
}
