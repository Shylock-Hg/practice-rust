fn main() {
        let r = Rect {
                weight: 30,
                height: 30,
        };
        // println!("The rect is {}", r);  // need std::fmt::Display trait
        println!("The rect is {:?}", r);
        println!("The area is {}", area(r));
}

#[derive(Debug)]  // impl debug format output
struct Rect {
        weight: u32,
        height: u32,
}

fn area(rect: Rect) -> u32 {
        return rect.weight*rect.height;
}
