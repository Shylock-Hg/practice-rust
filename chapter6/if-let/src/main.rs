fn main() {
        // match only one expression
        // synopsis -- match expression {pattern, pattern...}
        match Some(3) {
                Some(i) => println!("Some({}) fit.", i),
                _ => (),
        }

        // match multi expression
        // synopsis -- if let pattern = expression
        if let Some(i) = Some(3) {
             println!("Some({}) fit.", i);
        //if let None = Some(3) {
        //       println!("Some fit.");
        } else if let 3 = 3 {
                println!("Value fit.");
        } else {
                println!("Not fit.");
        }
}
