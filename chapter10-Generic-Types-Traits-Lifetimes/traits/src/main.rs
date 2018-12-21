fn main() {
        let t = Tweet {
                username: String::from("Shylock"),
                content: String::from("Hello World!"),
                replay: false,
                retweet: false,
        };
        println!("New tweet `{}`.", t.summarize());  // call overwrited implementation
        println!("Author is `{}`.", t.author());
        notify(t);

        let a = Article {
                headline: String::from("The day!"),
                location: String::from("xxx road xxx"),
                author: String::from("Shylock"),
                content: String::from("To be or not to be."),
        };
        println!("New article `{}`.", a.summarize());  // call default implementation
        println!("Author is `{}`.", a.author());
        notify(a);

        // notify(3);  //error not implement Summary trait
        let t2 = get_summerizable();
        notify(t2);

        let v = vec![-22, 3, 4, 0, 200];
        println!("The largest is {}.", largest(&v));

        let s = 3.to_string();
        println!("The string of 3 is {}.", s);
}

trait Summary {
        // fn summarize(&self) -> String;  // trait signature
        fn summarize(&self) -> String {   // default implementation
                String::from("Read more...")
        }

        fn author(&self) -> String;
}

struct Article {
        headline: String,
        location: String,
        author: String,
        content: String,
}

impl Summary for Article {
        fn author(&self) -> String {
                format!("@{}", self.author)
        }
}  // default implementation

/*
impl Summary for Article {
        fn summarize(&self) -> String {
                format!("{} by {} in {}", self.headline, self.author, self.location)
        }
}
*/

struct Tweet {
        username: String,
        content: String,
        replay: bool,
        retweet: bool,
}

impl Summary for Tweet {
        fn summarize(&self) -> String {
                format!("{} : {}", self.username, self.content)
        }

        fn author(&self) -> String {
                format!("@{}", self.username)
        }
}

// using trait declare parameter
/*
fn notify(text: impl Summary) {
        println!("The summary is `{}`.", text.summarize());
}
*/

// trait bound
fn notify<T: Summary> (text: T) {
        println!("The summary is `{}`.", text.summarize());
}

// specify multiple traits with + operator
// fn notify (text: impl Summary + Display);
// fn notify<T: Summary + Display> (text: T);

// where alternate trait bound
// fn some_func (first: T, second: U)
//      where T: Clone + Display,
//            U: Clone + Debug,

// return trait
fn get_summerizable() -> impl Summary {
        Tweet {
                username: String::from("Shylock"),
                content: String::from("Hello World!"),
                replay: false,
                retweet: false,
        }
}

// return trait from various variants
/*
fn get_summerizable(switch: bool) -> impl Summary {  // error for mismatch return type
        if switch {
                Tweet {
                        username: String::from("Shylock"),
                        content: String::from("Hello World!"),
                        replay: false,
                        retweet: false,
                }
        } else {
                Article {
                        headline: String::from("The day!"),
                        location: String::from("xxx road xxx"),
                        author: String::from("Shylock"),
                        content: String::from("To be or not to be."),
                }
        }
}
*/

// using trait for largets
fn largest<T: Copy + PartialOrd>(v: &[T]) -> T {
        let mut temp = v[0];
        for &item in v.iter() {
                if temp < item {
                        temp = item;
                }
        }
        temp
}

// implement a trait for another trait
// impl<T: Display> ToString for T {}  // implement ToString trait for type implemented Display trait
