fn main() {
        print_num(3);  // statement
        println!("{}", add_two(2, 3));  // expression
}

// simple function without evaluated value
fn print_num(num: i32) {
        // Statements are instructions that perform some action and do not return a value.
        println!("{}", num);
}

// function with evaluated value
fn add_two(a: i32, b:i32) -> i32 {
        println!("{} + {}!", a, b);
        // Expressions evaluate to a resulting value. Let’s look at some examples.
        a+b  // must in last of function body(or expression body) {} with out semicolon
}
