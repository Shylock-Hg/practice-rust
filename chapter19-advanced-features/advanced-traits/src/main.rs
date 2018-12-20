fn main() {
        // the same named method of struct / traits
        let human = Human;
        human.fly();
        Pilot::fly(&human);
        Wizard::fly(&human);

        // the same named associated functions call by scope prefix directly
        Dog::shout();
        <Dog as Animal>::shout();
        // <Type as Trait>::func(if_method, next_arg ...);

        let p = Point {
                x: -3,
                y: 44,
        };
        p.outline_print();

        let t = Wrapper(vec![
                String::from("Hello World!"),
                String::from("Shylock"),
                String::from("Tcath2s")]);
        println!("{}", t);
}

// Associated Types in trait
// Such as bellow:
pub trait Iterator {
        type Item;  // assocaited type
        fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
        count: i32,
}

impl Iterator for Counter {
        type Item = i32;  // impl associated type

        fn next(&mut self) -> Option<Self::Item> {
                Some(self.count)
        }
}

/*
// compare to generic parameters
pub trait Iterator<T> {  // this will generate multi specific trait
        fn next(&mut self) -> Option<T>;
]
*/

// default value of generic parameters
// To extend a type without breaking existing code
// To allow customization in specific cases most users won’t need
pub trait Add<RHS=Self> {  // Self means the type impl the trait in fact
        type Output;
        fn add(&self, rhs: RHS) -> Self::Output;
}

// Same named method in traits and struct
trait Pilot {
        fn fly(&self);
}

trait Wizard {
        fn fly(&self);
}

struct Human;

impl Human {
        fn fly(&self) {
                println!("human fly!");
        }
}

impl Pilot for Human {
        fn fly(&self) {
                println!("pilot fly!");
        }
}

impl Wizard for Human {
        fn fly(&self) {
                println!("wizard fly!");
        }
}

// Same associated function in traits and struct
struct Dog;

impl Dog {
        fn shout() {
                println!("Dog shout!");
        }
}

trait Animal {
        fn shout();
}

impl Animal for Dog {
        fn shout() {
                println!("Animal shout!");
        }
}

use std::fmt;

// Supertrait
trait OutlinePrint: fmt::Display {  // trait OutlinePrint relay on trait Display
        fn outline_print(&self);
}

struct Point {
        x: i32,
        y: i32,
}

impl OutlinePrint for Point {
        fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();

                println!("{}", "*".repeat(len+4));
                println!("*{}*", " ".repeat(len+2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len+2));
                println!("{}", "*".repeat(len+4));
        }
}

impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
        }
}


// Newtype pattern
// 1. manually impl traits for wrapper type
// 2. impl Deref trait for wrapper 
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
        }
}
