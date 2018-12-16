use std::rc;
use std::cell::RefCell;

fn main() {
        // combine Rc<T> and RefCell<T>
        let value = rc::Rc::new(RefCell::new(5));
        let a = rc::Rc::new(List::Cons(rc::Rc::clone(&value), rc::Rc::new(List::Nil)));
        let b = List::Cons(rc::Rc::new(RefCell::new(6)), rc::Rc::clone(&a));
        let c = List::Cons(rc::Rc::new(RefCell::new(10)), rc::Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("A is {:?}!", a);
        println!("B is {:?}!", b);
        println!("C is {:?}!", c);
}

// Use Rc<T> when we want to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish using the data last.
// only in single-thread scenarios

#[derive(Debug)]
enum List<T> {
        // combine Rc<T> and RefCell<T>
        Cons(rc::Rc<RefCell<T>>, rc::Rc<List<T>>),
        Nil,
}


// Interior mutability is a design pattern in RUST that allows you to mutate data even when there are immutable references to that data

// With reference and Box<T>, the borrowing rules' invariants are enforced at compile time.
// With RefCell<T>, these invariants are enforced at runtime.

// RefCell<T> is only in single-thread scenarios.

// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<"T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<"T> is immutable.

