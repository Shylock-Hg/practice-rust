extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

const LOWER : u32 = 0;
const UPPER : u32 = 99;

fn main() {
        println!("Hello Guess!");


        // generate random target number
        let target = rand::thread_rng().gen_range(LOWER, UPPER);
        
        loop {
                // read input line
                println!("Please input your guess:");
                let mut guess = String::new();
                io::stdin().read_line(&mut guess)
                        .expect("Read input failed!");
                
                // try to convert input string to number
                // fail continue
                let guess: u32 = match guess.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                };
                
                println!("You guess {}!", guess);
                
                // compare guess and target
                //
                match guess.cmp(&target) {
                        Ordering::Less => println!("Too less!"),
                        Ordering::Greater => println!("Too greater!"),
                        Ordering::Equal => {
                                println!("Win!");
                                break;
                        },
                };
        }
}
