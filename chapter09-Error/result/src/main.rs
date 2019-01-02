// enum Result<T, E> {
//      Ok(T),
//      Err(E),
// }

use std::fs::File;
use std::io::ErrorKind;

fn main() {
        // println!("{:?}", result);
/*
        let result = File::open("Hello.txt");
        let f = match result {
                Ok(file) => file,
                Err(error) => {panic!("{:?}", error)},  // panic
        };
*/
/*
        let result = File::open("Hello.txt");
        let f = match result {
                Ok(file) => file,
                Err(error) => match error.kind() {
                        ErrorKind::NotFound => match File::create("Hello.txt") {
                                Ok(fc) => fc,
                                Err(e) => panic!("{:?}", e),
                        },
                        other_error => panic!("{:?}", other_error),
                },
        };
*/
        // closures
        // map_err and unwrap_or_else
/*
        let f = File::open("Hello.txt").map_err(|error| {
                if error.kind() == ErrorKind::NotFound {
                        File::create("Hello.txt").unwrap_or_else(|error| {
                                panic!("{:?}", error);
                        })
                } else {
                        panic!("{:?}", error);
                }
        });
*/

        // shortcut -- unwrap or expect function to Result
        // let result = File::open("Hello.txt").unwrap();  // panic
        // let result = File::open("Hello.txt").expect("Failed to open Hello.txt!");

        read_username_from_file().unwrap();
}

use std::io;
use std::io::Read;
/*
fn read_username_from_file() -> Result<String, io::Error> {
        let result = File::open("Hello.txt");
        let mut f = match result {
                Ok(file) => file,
                Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
                Ok(_) => {println!("{}", s); Ok(s)},
                Err(e) => Err(e),
        }
}
*/

//  ? after Result
// when Ok evaluate expression value T from Result<T, E>
// when Err return the Value E from Result<T, E>
/*
fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("Hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        println!("{:?}", s);
        Ok(s)
}
*/
fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("Hello.txt")?.read_to_string(&mut s)?;
        println!("{:?}", s);
        Ok(s)
}
