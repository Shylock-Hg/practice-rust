fn main() {
        let mut t = AveragedCollection::new();
        t.push(33);
        t.push(11);
        println!("The average is {}!", t.average());
        t.push(3);
        println!("The average is {}!", t.average());
        t.pop();
        println!("The average is {}!", t.average());
}

// common conception in OOP: object, encapsulation and inheritance.

// encapsulation
struct AveragedCollection {
        nums: Vec<i32>,
        average: f64,
}

impl AveragedCollection {
        fn new() -> AveragedCollection {
                AveragedCollection {
                        nums: vec![],
                        average: 0.0,
                }
        }

        fn average(&self) -> f64 {
                self.average
        }

        fn push(&mut self, num: i32) {
                self.nums.push(num);
                self.update();
        }

        fn pop(&mut self) {
                let result = self.nums.pop();
                match result {
                        Some(_) => self.update(),
                        None => {},
                };
        }

        fn update(&mut self) {
                let sum: i32 = self.nums.iter().sum();
                self.average = sum as f64 / self.nums.len() as f64;
        }
}

// inheritance and polymorphism
// Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.
