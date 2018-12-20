fn main() {
        let mut num = 3;
        apply(succ, &mut num);
        println!("{}", num);

        // tuple constructing in fact function
        let list_of_status: Vec<Status> = (0..28).map(Status::value).collect();
        println!("{:?}", list_of_status);
}

// Function pointer -- fn
fn succ(num: &mut i32) {
        *num += 1;
}

fn apply(f: fn(&mut i32), num: &mut i32) {
        f(num)
}

// fn is type, Fn is trait
// fn impl all Fn, FnMut and FnOnce traits


// tuple constructe
#[derive(Debug)]
enum Status {
        value(i32),
        Stop,
}


// return function
fn returu_function() -> fn(&mut i32) {
        succ
}
// return closure
fn return_closure() -> Box<dyn Fn(&mut i32)> {
        Box::new(|num: &mut i32| {*num += 1;})
}

