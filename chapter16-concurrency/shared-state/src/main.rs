use std::sync::{Mutex, Arc};
use std::thread;
use std::rc::Rc;

fn main() {
        let m = Mutex::new(5);
        {
                let mut num = m.lock().unwrap();  // get shared state by lock
                *num = 6;
                println!("{}!", num);
        }
        println!("Hello, world!");

/*
        let m = Mutex::new(0);
        let mut handles = vec![];
        for _ in 0..9 {
                let handle = thread::spawn(move || {  // !! error for taking moved value
                        let mut num = m.lock().unwrap();
                        println!("The number is {}!", num);
                        *num += 1;});
                handles.push(handle);
        }

        for handle in handles {
                handle.join().unwrap();
        }
*/

/*
        // Rc<T> reference count operation not MT-safe not Atomic
        let m = Rc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..9 {
                let handle = thread::spawn(move || {  // !! error for Rc<T> is unsafe across threads
                        let m = Rc::clone(&m);
                        let mut num = m.lock().unwrap();
                        println!("The number is {}!", num);
                        *num += 1;});
                handles.push(handle);
        }

        for handle in handles {
                handle.join().unwrap();
        }
*/

        let m = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..9 {
                let m = Arc::clone(&m);
                let handle = thread::spawn(move || {  // !! error for Rc<T> is unsafe across threads
                        let mut num = m.lock().unwrap();
                        *num += 1;});  // mutex release while exit scope
                handles.push(handle);
        }

        for handle in handles {
                handle.join().unwrap();
        }
        println!("The result is {}!", m.lock().unwrap());
}

// mutex - mutual exclusion
// 1. must lock before access data
// 2. must release after access data

// RefCell/Rc -- Mutex/Arc
