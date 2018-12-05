fn main() {
        println!("V4 is {:?}", IpType::v4());
        let localhost = Ip {kind: IpType::V4, address: String::from("127.0.0.1")};
        let lo = Ip {kind: IpType::V6, address: String::from("::1")};
        println!("lo addr is {}", lo.address);


        let localhost1 = IpAddr::V4(String::from("127.0.0.1"));
        let lo1 = IpAddr::V6(String::from("::1"));
        println!("lo1 is {:?}", lo1);


        let msg = Message::Move {x: 2, y: 3};
        println!("Move msg is {:?}", msg);
        let msg2 = Message::Quit;
        println!("Quit msg is {:?}", msg2);


        // Option enum
        let x:i32 = 3;
        let y     = Some(3);
        // x + y; // error: no matched function
        // used in logic instead of calculus ususally
}

// define enum and using inside struct
#[derive(Debug)]
enum IpType{
        V4,  // unit struct
        V6,  // unit struct
}

impl IpType {
        fn v4() -> IpType {
                return IpType::V4;  // must specify scope
        }

        fn v6() -> IpType {
                return IpType::V6;
        }
}

struct Ip {
        kind: IpType,
        address: String,
}

// define enum not in default type
#[derive(Debug)]
enum IpAddr {
        V4(String),  // tuple struct
        V6(String),  // tuple struct
}

// define enum in various type
#[derive(Debug)]
enum Message {
    Quit,  // unit struct
    Move { x: i32, y: i32 },  // struct
    Write(String),  // tuple struct
    ChangeColor(i32, i32, i32),  // tuple struct
}

// Option enum
// enum Option<T> {
//      Some(T),  // tuple struct
//      None,  // unit struct
// }
