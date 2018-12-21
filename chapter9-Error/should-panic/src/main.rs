extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


#[derive(Debug)]
struct Guess {
        guess: i32,
}

impl Guess {
        const LOWER : i32 = 0;
        const UPPER : i32 = 99;

        // using panic when can't handle the error by patch code
        // a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code
        // 
        // The bad state is not something that’s expected to happen occasionally.
        // Your code after this point needs to rely on not being in this bad state.
        // There’s not a good way to encode this information in the types you use.
        // this case is not property
        fn from(g: i32) -> Guess {
                if Guess::LOWER > g || Guess::UPPER < g {
                        panic!("Guess value `{}` is out of range!", g);
                }
                Guess {
                        guess: g,
                }
        }

        fn value(&self) -> i32 {
                self.guess
        }
}

fn main() {
        println!("Hello Guess!");


        // generate random target number
        let target: i32 = rand::thread_rng().gen_range(Guess::LOWER, Guess::UPPER);
        
        loop {
                // read input line
                println!("Please input your guess between {} and {}:", Guess::LOWER, Guess::UPPER);
                let mut guess = String::new();
                io::stdin().read_line(&mut guess)
                        .expect("Read input failed!");
                
                // try to convert input string to number
                // fail continue
                let guess: i32 = match guess.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                };

                let guess = Guess::from(guess);
                
                println!("You guess {:?}!", guess);
                
                // compare guess and target
                //
                match guess.value().cmp(&target) {
                        Ordering::Less => println!("Too less!"),
                        Ordering::Greater => println!("Too greater!"),
                        Ordering::Equal => {
                                println!("Win!");
                                break;
                        },
                };
        }
}
