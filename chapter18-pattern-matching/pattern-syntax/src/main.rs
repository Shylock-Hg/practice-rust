fn main() {
        // 1. matching literal
        let x = 1;
        println!("{}", match x {
                1 => "one",
                2 => "two",
                3 => "three",
                _ => "others",
        });

        // 2. matching named variable
        let x = Some(50);
        let y = 10;
        match x {
                Some(50) => println!("Got 50."),
                Some(y) => println!("Match y {:?}.", y),  // create a variable named y when matched
                _ => println!("Others."),
        };

        // multi pattern
        let x = 1;
        match x {
                1 | 2 => println!("one or two"),  // union of patterns
                3 => println!("three"),
                _ => println!("others"),
        };

        // range pattern -- number and char
        let x = 1;
        match x {
                1...5 => println!("one to five"),
                _ => println!("others"),
        };

        // destructuring struct
        let p = Point {
                x: -11,
                y: 3,
        };
        let Point {x: a, y: b} = p;
        assert_eq!(a, -11);
        assert_eq!(b, 3);

        let Point {x, y} = p;  // variable must named as field name
        assert_eq!(x, -11);
        assert_eq!(y, 3);

        match p {  // variable must named as field name
                Point {x, y: 0} => println!("x is {}, y is 0.", x),
                Point {x: 0, y} => println!("x is 0, y is {}.", y),
                Point {x, y} => println!("x is {}, y is {}.", x, y),
        }

        // destructuring enum
/*
        let m = Message::Move{x: -22, y: 1};
        match m {
                Message::Quit => println!("Quit."),
                Message::Move{x, y} => println!("Move x {}, y {}.", x, y),
                Message::Write(s) => println!("Write {}.", s),
                Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        };
*/

        // destructuring nested struct and enum
        let m = Message::ChangeColor(Color::RGB(-2, 55, 255));
        match m {
                Message::Quit => println!("Quit."),
                Message::Move{x, y} => println!("Move x {}, y {}.", x, y),
                Message::Write(s) => println!("Write {}.", s),
                Message::ChangeColor(Color::RGB(r, g, b)) => println!("Change color to ({}, {}, {})", r, g, b),
                Message::ChangeColor(Color::HSV(h, s, v)) => println!("Change color to ({}, {}, {})", h, s, v),
        };

        // destructuring reference
        let points = vec![
                Point {x: 2, y: 3},
                Point {x: -1, y: 44},
                Point {x: 3, y: 2},
                Point {x: 5, y: 66},
        ];
        let sum_of_square: i32 = points.iter()
                .map(|&Point {x, y}| x*x + y*y)
                .sum();
        println!("sum_of_square is {}.", sum_of_square);

        // destructuring struct and tuple
        let ((feet, inch), Point {x, y}) = ((2 ,3), p);
        assert_eq!(a, -11);
        assert_eq!(b, 3);

        // ignore pattern
        let _ = 3;

        let mut setting_value = Some(3);
        let new_setting_value = Some(6);
        match (setting_value, new_setting_value) {
                (Some(_), Some(_)) => println!("Can't overwrite a exist settng value!"),
                _ => setting_value = new_setting_value,
        }
        assert_eq!(setting_value, Some(3));

        let numbers = (1, 2, 3, 4 ,5);
        match numbers {
                (first, _, third, _, fifth) => println!("{}, {}, {}!", first, third, fifth),
        }

        let _z = 3;  // ignore unused variable

        let s = Some(String::from("Hello World!"));
/*
        if let Some(_s) = s {  // !! error for move assignment here
                println!("Matched!");
        }
*/
        if let Some(_) = s {  // !! 
                println!("Matched!");
        }
        println!("{}", s.unwrap());

        // ignore remaning part
        let p = Point {
                x: 3,
                y: 4,
        };
        match p {
                Point {x, ..}  => println!("x is {}.", x),
        }

        let numbers = (1, 2, 3, 4, 5);
        match numbers {
                (first, .., last) => println!("first is {}, last is {}.", first, last),
        };

        // extra condition with the match guard
        let num = Some(4);
        match num {
                Some(i) if i < 5 => println!("Condition less than 5."),
                Some(i) => println!("Some({})", i),
                None => (),
        };

        // @ binding
        let msg = Msg::Hello {id: 5};
        match msg {
                Msg::Hello {id: id_binding @ 2...10} => println!("Binding to {}.", id_binding),
                Msg::Hello {id: 1...5} => println!("sencond range."),  // can't access value of id field
                Msg::Hello {id} => println!("id is {}.", id),
        }

        // legacy pattern
        let robot_name = &Some(String::from("Shylock"));
/*
        match robot_name {
                // match by reference
                // anti move by ref keyword
                &Some(ref s) => println!("Name is {}.", s),
                None => (),
        }
        println!("{:?}", robot_name);
*/
        match robot_name {  // fine tuned code
                Some(s) => println!("Name is {}.", s),
                None => (),
        }
        println!("{:?}", robot_name);
        // ref, ref mut is usefull in borrowing part in various mutability
}

struct Point {
        x: i32,
        y: i32,
}

/*
enum Message {
        Quit,
        Move{x:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32),
}
*/

enum Color {
        RGB(i32, i32, i32),
        HSV(i32, i32, i32),
}

enum Message {
        Quit,
        Move{x:i32, y:i32},
        Write(String),
        ChangeColor(Color),
}

enum Msg {
        Hello {id: i32},
}
