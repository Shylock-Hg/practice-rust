static mut COUNT: u32 = 0;  // unsafe access

fn add_count() {
        unsafe {
                COUNT += 1;
        }
}

fn main() {
        // 1. raw pointer
        // *const T, *mut T
        // different from reference or smart pointer in:
        //      1. Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
        //      2. Aren’t guaranteed to point to valid memory
        //      3. Are allowed to be null
        //      4. Don’t implement any automatic cleanup

        // create raw pointer from reference
        let mut num = 5;
        let p1 = & num as *const i32;
        let p2 = &mut num as *mut i32;
        unsafe {
                println!("P1 pointer to value {}!", *p1);
                println!("P2 pointer to value {}!", *p2);
        }
        
        // create raw pointer from address
        let address = 0x012345usize;
        let r = address as *const i32;

        // dangerous(); // !! error for call unsafe function in non unsafe block

        let mut s = vec![1, 2, 3, 4, 5, 6, 7];
        let (s1, s2) = split_at_mut(&mut s, 3);
        println!("Split to {:?}, {:?}.", s1, s2);

        let mut n = 0;
        unsafe {n = abs(-3);}  // FFI is always unsafe
        println!("abs(-3) is {}!", n);

        add_count();
        unsafe {
                println!("The COUNT is {}.", COUNT);
        }
}

// Why unsafe?
// 1. Static analysis is conservative
// 2. Computer is unsafe in fact, safe rules limit its functions.

// unsafe supperpower
// 1. Dereference a raw pointer.
// 2. Call unsafe function or method.
// 3. Access or modify a mutable static variable.
// 4. Implement a unsafe trait.
// the unsafe keyword only allow to four operations above.The RUST static checker works well too.

// better to wrap unsafe operations by safe abstraction

//By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.

// unsafe function/method
unsafe fn dangerous() {}

// wrap unsafe code part in safe function is the common safe abstraction
use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();  // slice lenght
        let ptr = slice.as_mut_ptr();  // raw pointer to slice
        assert!(len >= mid);

        unsafe {
                (slice::from_raw_parts_mut(ptr, mid),  // create slice from (raw pointer, length)
                        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
        }  // must notice the align of bytes
}

// FFI -- foreign function interface -- always unsafe
// extern keywords
extern "C" {  // "C" -- specify the ABI(Application Binary Interface) -- how function call in assembly level
        fn abs(input: i32) -> i32;  // specify FFI name and signatrue
}

// export to other language calling
#[no_mangle]
pub extern "C" fn call_from_c() {  // compile this to library(static/shared) to used by C language
        println!("Hellow C!");
}

// unsafe trait
// A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
unsafe trait Foo {
}

unsafe impl Foo for i32 {}
// such as Send or Sync when the part not impl Send or Sync, can only unsafe impl Send or Sync for entire type manually.
