use std::thread;
use std::time::Duration;

fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(
                move || {  // move to force take ownership of v
                        println!("{:?}", v);
                        for i in 0..9 {
                                println!("Hi number {} from the spawned thread!", i);
                                thread::sleep(Duration::from_millis(1));
                        }
                }
        );
        handle.join().unwrap();  // wait thread exit

        for i in 0..4 {
                println!("Hi number {} from the main thread!", i);
                thread::sleep(Duration::from_millis(1));
        }

}  // all threads exit when main thread exit

// Race conditions, where threads are accessing data or resources in an inconsistent order
// Deadlocks, where two threads are waiting for each other to release a resource the other thread has, preventing both threads from continuing.
// Bus that happen only certain situations and are hard to reproduce and fix reliably.

// 1:1 programing language threads model  -- rust only for fewer runtime cost
// M:N programing language threads model
