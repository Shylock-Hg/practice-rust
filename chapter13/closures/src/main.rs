use std::thread;
use std::time::Duration;

extern crate log;
extern crate stderrlog;
use log::{trace, debug, info, warn, error};


fn main() {
        // stderrlog
        stderrlog::new().module(module_path!()).verbosity(99).init().unwrap();

        generate_workout_print_lazy(10, 7);  // call
        generate_workout_print_lazy(30, 3);  // not call 
        generate_workout_print_lazy(30, 7);  // call

        // let c = |x| x;  // error for conflict type at time
        // let d = c(3);
        // let e = c(String::from("Hello World!"));

        let x = 4;
        // capture the context as immutable borrow so impl Fn trait
/*
        let is_equal_x = |y| y == x;
        // x = 5;  // can't modify for borrowed by capture
*/

        // force to take ownership(assignment in fact)
        // meanful for dynamic resource
        let is_equal_x = move |y| y == x;  // assignment here

        assert!(is_equal_x(4));
        println!("The x is {} now!", x);

        // FnOnce -- capture the context as assignment(take ownership)
        // FnMut -- capture the context as mutable reference
        // Fn -- capture the context as immutable reference
}


fn generate_workout_print(intensity: u32, random_number: u32) {
        let i = simulated_expensive_calculate(intensity);

        if intensity < 25 {
                println!(
                        "Push up {} times!",
                        i
                        );
                println!(
                        "Situp {} times!",
                        i);
        } else {
                if random_number == 3 {
                        println!("Take a break day!");
                } else {
                        println!(
                                "Run for {} minutes!",
                                i);
                }
        }
}

fn generate_workout_print_closure(intensity: u32, random_number: u32) {
        // optional annotation
        // automatic type of parameter and return
        // each closure has the unique anonymous type
        let closure = |intensity| {
                trace!("Calculating...");
                thread::sleep(Duration::from_secs(2));
                intensity
        };

        if intensity < 25 {
                println!(
                        "Push up {} times!",
                        closure(intensity)
                        );
                println!(
                        "Situp {} times!",
                        closure(intensity));
        } else {
                if random_number == 3 {
                        println!("Take a break day!");
                } else {
                        println!(
                                "Run for {} minutes!",
                                closure(intensity));
                }
        }
}

fn simulated_expensive_calculate(intensity: u32) -> u32 {
        trace!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        intensity
}

struct Cacher<T> 
        where T: Fn(u32)->u32 {
        calculate: T,
        value: Option<u32>,
}

impl<T> Cacher<T> 
        where T: Fn(u32)->u32 {

        fn new (cal: T) -> Cacher<T> {
                Cacher {
                        calculate: cal,
                        value: None,
                }
        }

        // the value is same after first evaluate
        // improve by HashMap<input, output>
        fn evaluate(&mut self, arg: u32) -> u32 {
                match self.value {
                        Some(v) => v,
                        None => {
                                let v = (self.calculate)(arg);
                                self.value = Some(v);
                                v
                        }
                }
        }
}

fn generate_workout_print_lazy(intensity:u32, random_number: u32) {
        // optional annotation
        // automatic type of parameter and return
        // each closure has the unique anonymous type
        let closure = |intensity| {
                trace!("Calculating...");
                thread::sleep(Duration::from_secs(2));
                intensity
        };
        let mut cache = Cacher::new(closure);

        if intensity < 25 {
                println!(
                        "Push up {} times!",
                        cache.evaluate(intensity));
                println!(
                        "Situp {} times!",
                        cache.evaluate(intensity));
        } else {
                if random_number == 3 {
                        println!("Take a break day!");
                } else {
                        println!(
                                "Run for {} minutes!",
                                cache.evaluate(intensity));
                }
        }
}
