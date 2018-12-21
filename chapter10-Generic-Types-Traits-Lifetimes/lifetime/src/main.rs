fn main() {
        {
                let r;               // ---------+ b
                {                    //          |
                        let x = 5;   // -----+a  |
                        r = &x;      //      |   |
                }                    // -----+   | 
                // println!("{}", r);//          |     // error null reference
                                     //          |
                // borrow checker    //          |
                                     //          |
        }                            // ---------+

/*
        let s1 = String::from("abcd");
        let result;
        {
                let s2 = String::from("xyz");
                result = longest(s1.as_str(), s2.as_str());
        }
        println!("The longest string is {}.", result);  // error for lifetime
*/

/*
        let first_sentence;
        {
                let novel = String::from("Call me Ishmael. Some years ago...");
                first_sentence = novel.split('.').next().unwrap();
        }
        let i = ImportantExcerpt {part: first_sentence};  // error for lifetime
        println!("{}", i.part);
*/

        let s: &'static str = "I have a static lifetime.";

        let s1 = String::from("abcd");
        let result;
        {
                let s2 = String::from("xyz");
                result = longest_with_an_announcement(
                        s1.as_str(), s2.as_str(), "Hello");
        }
        println!("The longest string is {}.", result);  // error for lifetime
}
// !! this tell compiler the reference in same scope with same lifetime specifier must both live as long as the generic lifetime
// !! the lifetime specifier has lifetime same as the minimum of scope of all references passed in
// the reference x, y, retval must live at time
// passed in call `input lifetime`, passed out called `output lifetime`
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // expecte lifetime specifier
        if x.len() > y.len() {
                x
        } else {
                y
        }
}


// &i32
// &'a i32 reference with lifetime a
// &'a mut i32 mutable reference with lifetime a


// in struct
struct ImportantExcerpt<'a> {
        // the reference of struct instance has lifetime same as the minimum of scope of all reference member
        part: &'a str,  // must specify lifetime
}


// lifetime elision rules
// not need explicit lifetime specifier in some situation
// the now elision rules
// 1. each reference parameter get its own lifetime specifier,
//      such as fn longest<'a, 'b>(x: &'a str, y: &'b str)
// 2. appliies when there is exactly one input lifetime
//    specify input lifetime to all output lifetimes
//      such as fn first_word<'a>(s: &'a str) -> 
// 3. in method the lifetime of self is specified to all output lifetimes

// lifetime in method
impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {  // apply first elision rule
                3
        }

        // apply first and third rules
        fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
        }
}


// static lifetime -- 'static long as the process

// generic type parameter, trait bound and lifetime specifier
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
        where T: Display, {
        println!("Announcement {}", ann);
        if x.len() > y.len() {
                x
        } else {
                y
        }
}
