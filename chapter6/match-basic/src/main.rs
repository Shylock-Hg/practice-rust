fn main() {
        let p = Coin::PENNY;
        let d = Coin::DIME;
        let q = Coin::QUARTER(UsState::ALABAMA);
        println!("Cents of PENNY is {}", p.cents());
        println!("Cents of DIME is {}", d.cents());
        println!("Cents of QURTER is {}", q.cents());

        println!("Succ 3 is {:?}", succ(Some(3)));
        println!("Succ None is {:?}", succ(None));
        println!("Succ 0 is {:?}", succ(Some(0)));

        underline(0);
        underline(23);
}

#[derive(Debug)]
enum UsState {
        ALABAMA,
        ALASKA,
}

enum Coin {
        PENNY,
        NICKEL,
        DIME,
        QUARTER(UsState),
}

impl Coin {
        fn cents (&self) -> u32 {
                // match the first fit
                match self {
                        // value match in this case
                        Coin::PENNY => 1,  // must specify scpoe
                        Coin::NICKEL => 5,
                        Coin::DIME => 10,
                        // partial value match in this case
                        Coin::QUARTER(state) => {
                                println!("Quarter from {:?}", state);
                                25
                        },
                }
        }
}

fn succ(x: Option<i32>) -> Option<i32> {
        match x {
                None => None,
                Some(x) => Some(x+1),
        }
}

fn underline(i: u8) {
        match i {
                0 => {println!("Zero");},
                1 => {println!("One")},
                _ => (),  // all the other case not fit now
        };
}
