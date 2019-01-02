fn main() {
        let s = String::from("Hello World!\n");
        let w = first_word(&s);
        println!("{}", w);
        // s.clear();  // error -- try to take mutable reference from immutable var
}

fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                        return &s[..i];  //  return the reference to slice
                }
        }

        return &s[..];  //  return the reference to slice
}

//  origin string[ptr, len, cap] -> | |
//                                  | |
//                                  | |
//                                  | |
//                                  | |
//  slice reference[ptr, len] -->   | |
//                                  | |
//                                  | |
//                                  | |
