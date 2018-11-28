fn main() {
        let num = 3;
        let _a = if num < 5 {
                println!("Condition is true");
                true
        } else {
                println!("Condition is false");
                false
        };

        let b = if num > 5 {
                true
        } else {
                false
        };
        println!("Conditon is {}", b);
}
