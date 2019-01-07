fn main() {
        let s1 = String::from("Hello World!\n");
        let len = calculate_lenth(& s1);  // pass reference instead of move
        println!("Length is {}!\n", len);

        let mut s2 = String::from("Hello ");
        change_str(&mut s2);  // pass mutable reference
        println!("{}", s2);

        // error: only one mutable reference to a variable in one specific scope
        // let &mut s3 = s2;
        // let &mut s4 = s2;
        
        // let s5 = dangle();
}

fn calculate_lenth(s: & String) -> usize {
        s.len()  // s reference to s1 in main function scope in this case
}

fn change_str(s: &mut String) {
        s.push_str(":)\n");  // s reference to s2 in main function scope in this case
}

/*  error: reference to unexist variable
fn dangle() -> &String {
        let s = String::from("Hello World!\n");
        &s
}
*/

// ! reference is the one implementation of borrow(others such as: Rc, slice or other user-defined RC-based smart pointers)


// ! In implementation
//   1. Borrower(such as reference and slice etc.) don't impl Drop trait, Owner(such as Box and String etc...) impl Drop trait.
//   2. The RC-based smart pointer are owner and brrower at the same time.(The drop method will be involved when strong reference decrease to zero).
