use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// derive just works on struct or enum
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
        // use custom derive macro
        Pancakes::hello_macro();
}

// Macros
// Declarative macros with macro_rules:
// Procedural macros, which come in three kinds:
//      1. Custom #[derive] macros.
//      2. Attribute-like macros.
//      3. Function-like macros.

// Metaprogramming
// #[derive(Macro)] to auto generate trait implementation

// macro invoved in compile time
// macro can take variable number parameters
// must define or bring macros into scope before you call them in file.


// Declarative Macros with macro_rules!
#[macro_export]  // make macro available to the crate imported
macro_rules! vec {
        ( $( $x:expr ),* ) => {  // macro pattern machting RUST code structure
                {
                        let mut temp_vec = Vec::new();
                        $(
                                temp_vec.push($x);
                        )*
                        temp_vec
                }
        }
}

// vec![1, 2, 3], the $x:expr match 1, 2, 3 in three times, the * means matching occurs >=0 times.


// attribute-like macro
// #[route(GET, "/")]
// fn index() {

// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream;  // attr -- The attribute, GET, "/" in above case; item -- the origin source code, the whole fn index() {...} function in this case.



// function-like macros
// let sql = sql!(SELECT * FROM posts WHERE id=1);

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream;
