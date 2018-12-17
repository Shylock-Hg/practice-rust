use std::cell::RefCell;
use std::rc::Rc;

fn main() {
        let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
        println!("Initial reference count of a is {}!", Rc::strong_count(&a));
        println!("Tail of a is {:?}", a.tail());
        // println!("Tail of a.tail() is {:?}", a.tail().unwrap().borrow().tail());

        let b = Rc::new(List::Cons(6, RefCell::new(Rc::clone(&a))));
        println!("Then the reference count of a is {}!", Rc::strong_count(&a));
        println!("Initial reference count of b is {}!", Rc::strong_count(&b));

        if let Some(tail) = a.tail() {
                *tail.borrow_mut() = Rc::clone(&b);
        }
        println!("Then the reference count of b is {}!", Rc::strong_count(&b));

        // cycle reference -- a->Cons->b->Cons->a
        // println!("Then the tail of a is {:?}!", a.tail());

        // solution 1. Testing
        //          2. Refactor data structure to avoid cycle
        //          3. Using non-ownership reference


        // Weak
        let leaf = Node::new(3);
        println!("Leaf is {:?}!", leaf);
        {
                let branch = Node::new(4);
                println!("Branch is {:?}!", branch);
                println!("The branch strong_count {}, weak_count {}!",
                        Rc::strong_count(&branch), Rc::weak_count(&branch));

                branch.children.borrow_mut().push(Rc::clone(&leaf));  // branch own the leaf
                println!("Branch is {:?}!", branch);
                println!("The leaf strong_count {}, weak_count {}!",
                        Rc::strong_count(&leaf), Rc::weak_count(&leaf));
                println!("The branch strong_count {}, weak_count {}!",
                        Rc::strong_count(&branch), Rc::weak_count(&branch));

                *(leaf.parent.borrow_mut()) = Rc::downgrade(&branch);  // leaf don't own the branch
                println!("Parent of leaf is {:?}!", leaf.parent.borrow().upgrade());
                println!("The branch strong_count {}, weak_count {}!",
                        Rc::strong_count(&branch), Rc::weak_count(&branch));
        }

        println!("Parent of leaf is {:?}!", leaf.parent.borrow().upgrade());
}


#[derive(Debug)]
enum List<T> {
        Cons(T, RefCell<Rc<List<T>>>),
        Nil,
}

impl<T> List<T> {
        fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
                match self {
                        List::Cons(_, tail) => Some(tail),
                        List::Nil => None,
                }
        }
}

use std::rc::Weak;


#[derive(Debug)]
struct Node<T> {
        value: T,  // value
        parent: RefCell<Weak<Node<T>>>,  // parent
        children: RefCell<Vec<Rc<Node<T>>>>,  // children
}

impl<T> Node<T> {

        fn new(v: T) -> Rc<Node<T>> {
                Rc::new(Node {
                        value: v,
                        parent: RefCell::new(Weak::new()),
                        children: RefCell::new(vec![]),
                })
        }
}
