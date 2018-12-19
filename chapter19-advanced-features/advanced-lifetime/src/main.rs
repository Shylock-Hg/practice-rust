fn main() {
        // Lifetime Subtyping
        let c = Context("Hello World!");
        println!("{:?}", parse_context(c));
        
        let num = 5;
        let obj = Box::new(Ball {diameter: &num}) as Box<dyn Red>;  // Box<dyn Red + 'a>
}

// Lifetime subtyping: ensures that one lifetime outlives another lifetime.
// Lifetime bounds: specifies a lifetime for a reference to generic type.
// Inference of trait object lifetimes: allows the compiler to infer trait object lifetimes and when they need to be specified.
// The anonymous lifetime: makeing elision more obvious.


// Lifetime Subtyping
// annotate the lifetime for reference to Context and reference to String
/*
struct Context<'a>(&'a str);

struct Parser<'a> {
        context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
        fn parse(& self) -> Result<(), & str> {
                Err(& self.context.0[..])
        }
}
*/
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
        context: &'c Context<'s>,
}

// !! ok in now
impl<'c, 's> Parser<'c, 's> {  // !! 's: 'c means lifetime of 's >= lifetime of 'c
        fn parse(& self) -> Result<(), &'s str> {
                Err(& self.context.0[..])
        }
}

fn parse_context(context: Context) -> Result<(), &str> {
        Parser {context: &context}.parse()
}


// Lifetime Bounds
// T could be reference or hold references, each of which could have their own lifetimes, so can't sure T will live as long as 'a
// struct Ref<'a, T> (&'a T);  // !! ok in now
// struct Ref<'a, T: 'a> (&'a T);  // !! ok in now


// Inference of Trait Object Lifetime
// 1. The default lifetime of a trait object is 'static
// 2. With &'a Trait or &'a mut Trait, the default lifetime of the trait object is 'a.
// 3. with single T: 'a clause, the default lifetime of the trait object is 'a.
// 4. With multiple clause like T: 'a, there is no default lifetime; we must be explicit
// note: As with the other bounds, the syntax adding a lifetime bound means that any implementor of the Red trait that has references inside the type must have the same lifetime specified in the trait object bounds as those reference. 
trait Red {}

struct Ball<'a> {
        diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}



// Anonymous lifetime
struct StrWrap<'a>(&'a str);

/*
fn foo<'a> (string: &'a str) -> StrWrap<'a> {
        StrWrap(string)
}
*/
// also works as bellow
fn foo(string: & str) -> StrWrap<'_> {
        StrWrap(string)
}

/*
// verbose
impl<'a> fmt::Debug for StrWrap<'a> {}

// elided
impl fmt::Debug for StrWrap<'_> {}
*/
