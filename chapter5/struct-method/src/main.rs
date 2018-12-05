fn main() {
        let r = Rect {weight: 32, height: 32};
        println!("Rect is {:?}", r);
        println!("Rect area is {}", r.area());

        let r1 = Rect {weight: 32, height:32};
        println!("Is r == r1: {}", Rect::cmp(&r, &r1));  // call associated function

        println!("Can r hold r1: {}", r.can_hold(&r1));
}

#[derive(Debug)]
struct Rect {
        weight: u32,
        height: u32,
}

impl Rect {
        // method of Rect
        // same as plain function except
        // 1. in scope of struct
        // 2. pass &self as first argument
        fn area(&self) -> u32 {
                return self.weight*self.height;
        }

        // method of Rect with more parameters
        fn can_hold(&self, b: &Rect) -> bool {
                return self.weight >= b.weight && self.height >= b.height;
        }

        // associated funtion
        // same as plain function except
        // 1. in scope of struct
        fn cmp(a: &Rect, b: &Rect) -> bool {
                return a.weight ==  b.weight && a.height == b.height;
        }
}
