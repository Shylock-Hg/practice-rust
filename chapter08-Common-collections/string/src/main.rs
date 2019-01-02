fn main() {
        let mut s = String::new();
        // create string from literal
        let s = "Hello World!".to_string();
        let mut s = String::from("Hello World!");

        // update String
        s.push_str("Job");
        println!("{}", s);
        s.push('X');
        println!("{}", s);

        let s1 = String::from("Hello ");
        let s2 = String::from("World!");
        let s3 = s1 + &s2;  // s1 moved to s3
        // println!("{}", s1);  // moved
        println!("{}", s2);
        println!("{}", s3);

        // invalid index String
        // 0.as bytes, 1.as charecters, 3.grapheme cluster
        // println!("{}", s1[0]);  // can't index one item

        // slice as bytes
        let s = "人民群众";
        let s = &s[0..6];  // bytes[0..6]
        println!("{}", s);
        // let s = &s[0..4];  // bytes[0..4]  panic for index a whole character
        // println!("{}", s);

        // iteration
        for c in "为人民币服务".chars() {  // iterate as characters
                println!("{}", c);
        }
        for c in "为人民币服务".bytes() {  // iterate as bytes
                println!("{}", c);
        }
}
