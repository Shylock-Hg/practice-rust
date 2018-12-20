fn main() {
        let x: i32 = 3;
        let y: Kilometer  = 11;
        println!("{}", x + y);

        // str(DST) not &str(SST)
        // let s1: str = "Hello World!";  // !! error for define DST variable

        // interior dynamically sized type
        let s2: &str = "Hello World!";  // !! SST, maybe {ptr: usize, len: usize}
}

// using Newtype for safe and abstraction
// 1. anti confuse using
// 2. expose interface and hiden interal implementation

// Type Synonyms
type Kilometer = i32;
// used to alias long type
type Thunck = Box<dyn Fn() + Send + 'static>;
// type Result<T> = Result<T, std::io::Error>;



// Empty Type -- means function never return
// we can't create a value typed !
// this called diverging functions
fn bar() -> ! {
        loop{}
}
// The formal way of describing this behavior is that expressions of type ! can be coerced into any other type.



// DST(dynamically sized types) or SST(static sized types)

/*
// they are same
fn generic<T>(p: T) -> T {
}
fn generic<T: sized>(p: T) -> T {}
*/

fn generic<T: ?Sized>(p: &T) -> &T {  // T maybe not Sized, so just can be used by reference
        p
}
