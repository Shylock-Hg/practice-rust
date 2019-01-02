fn main() {
        // Absolute path
        crate::sound::instrument::clarinet();
        // Relative path
        self::sound::instrument::clarinet();
        sound::instrument::clarinet();

        instrument::clarinet();  // also ok
        use crate::sound::instrument;
        instrument::clarinet();

        use self::sound::instrument::clarinet;
        clarinet();

        let mut v = plant::Vegetable::new("squash");
        v.name = String::from("butternut squash");
        println!("{} is delicious.", v.name);
        // error for access private member in struct
        // println!("id of {} is {}.", v.name, v.id);

        let a = menu::Appetizer::Soup;
        let b = menu::Appetizer::Salad;
        println!("{:?}", a);

        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(1, 2);
        println!("{:?}", map.get(&1));

        // renaming
        use std::collections::HashMap as unorderd_map;
        let mut map: unorderd_map<&str, &str> = unorderd_map::new();
        map.insert("Hello", "World");
        println!("{:?}", map.get("Hello"));
        if let Some(i) = map.get("Hello") {
                println!("{}", i);
        } else {
                println!("None");
        }
        if let Some(i) = map.get("XXX") {
                println!("{}", i);
        } else {
                println!("None", );
        }

        // call re-export symbol in new scope
        performance_group::instrument::clarinet();

        // use externel crate
        // 1. specify in Cargo.toml
        // 2. specify use
        use rand::Rng;
        let rnum = rand::thread_rng().gen_range(0, 99);

        // use list
        use std::{cmp::Ordering, io};
        // import all in a scope
        use std::collections::*;
}


//  1. A crate is a binary or library.
//  2. The crate root is a source file that is used to know how to build a crate.
//  3. A package has a Cargo.toml that describes how to build one or more crates. At most one crate in a package can be a library.

// private default for all symbol

// function
mod sound {
        pub mod instrument {
                pub fn clarinet() {
                        // Function body code goes here
                        super::breathe_in();
                }
        }

        fn breathe_in() {
        }
}


// struct
mod plant {
        pub struct Vegetable {
                pub name: String,
                id: i32,
        }

        impl Vegetable {
                pub fn new(name: &str) -> Vegetable {
                        Vegetable {
                                name: String::from(name),
                                id: 1,
                        }
                }
        }
}

// enum
mod menu {
        #[derive(Debug)]
        pub enum Appetizer {
                Soup,
                Salad,
        }
}

// re-export symbol

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

