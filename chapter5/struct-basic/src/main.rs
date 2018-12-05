fn main() {
        let u = User {name: String::from("Shylock"),
                email: String::from("tcath2s@gmail.com"),
                sign_in_count: 33,
                active: true,
        };

        println!("User name is {}", u.name);

        //  can only specify mutable to entire struct var
        let mut u2 = User {name: String::from("Shylock"),
                email: String::from("emma@gmail.com"),
                sign_in_count: 33,
                active: true,
        };

        u2.name = String::from("Emma");
        println!("User name is {}", u2.name);

        // struct updating syntax follow the assigment rules
        // !! moving for assignment as default
        // !! copying for var without dynamic resource
        let u3 = User {
                name: String::from("Tom"),
                ..u2
        };
        println!("User name is {}", u2.name);  // not moved
        // println!("Email is {}", u2.email);  // error !! moved u2.email
        println!("Active is {}", u2.active);  // !! copy for var without dynamic resource
        println!("Email is {}", u3.email);  // moved from u2
        // u3.email = String::from("who@gmail.com");  // error u3 specified immutable

        // unit struct
        struct Unit{};  // often used for traits


        // tuple struct as named tuple in fact
        struct Color(u8, u8, u8);  // (r, g, b)
        let c = Color(33, 22, 66);
        println!("Red attribute is {}", c.0);
}

// can have a reference field too, but need to specify lifetime
struct User {
        name: String,  //  can't specify specific field to mutable
        email: String,
        sign_in_count: u64,
        active: bool,
}
