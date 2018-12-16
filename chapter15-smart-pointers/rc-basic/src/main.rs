use std::rc;

fn main() {

        let a = rc::Rc::new(List::Cons(5, rc::Rc::new(List::Cons(10, rc::Rc::new(List::Nil)))));
        let b = List::Cons(3, rc::Rc::clone(&a));
        println!("{}.", rc::Rc::strong_count(&a));
        let c = List::Cons(4, rc::Rc::clone(&a));
        println!("{}.", rc::Rc::strong_count(&a));
}

// Use Rc<T> when we want to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish using the data last.
// only in single-thread scenarios


enum List<T> {
        Cons(T, rc::Rc<List<T>>),
        Nil,
}

