fn main() {
        let v: Vec<i32> = Vec::new();  // i32
        let mut v = vec![1, 2, 3];  // i32
        println!("v[0] is {}", v[0]);

        // push value
        v.push(4);
        // get value
        println!("v[3] is {}", v[3]);
        // println!("v[100] is {}", v[100]);  // panic
        if let Some(value) = v.get(3) {
                println!("v.get(3) is {}", value);
        } else {
                println!("v.get(3) is None");
        }
        match v.get(3) {
                None => println!("v.get(3) is None"),
                Some(value) => println!("v.get(3) is {}", value),
        }

        let first = &v[0];
        // v.push(5);  // immutable reference
        println!("v[0] is {}", first);
        v.push(6);

        // iterate collection
        for item in &v {
                println!("Item is {}", item);
        }
        for item in &mut v {
                *item += 50;
        }
        for item in &v {
                println!("Item is {}", item);
        }

        // using enum to store various types values
        // or using trait
        #[derive(Debug)]
        enum SpreedSheetCell {
                Int(i32),
                Float(f32),
                Text(String),
        };
        let mut v = vec![SpreedSheetCell::Int(3),
                SpreedSheetCell::Float(3.3),
                SpreedSheetCell::Text(String::from("Hello"))];
        println!("{:?}", v[0]);
        match v[1] {
                SpreedSheetCell::Float(value) => println!("v[1] is {}", value),
                _ => (),
        }
        v.push(SpreedSheetCell::Int(4));
        match v[3] {
                SpreedSheetCell::Int(value) => println!("v[3] is {}", value),
                _ => (),
        }

        v.pop();
/*
        match v[3] {  // out of range
                SpreedSheetCell::Int(value) => println!("v[3] is {}", value),
                _ => (),
        }
*/
        
}
