use std::boxed::Box;

fn main() {
/*
        let mut s = oop_gui::Screen {
                components: vec![],
        };
        let b = Box::new(oop_gui::Button {
                weight: 33,
                height: 44,
                label: String::from("Button"),
        });
        let b2 = Box::new(oop_gui::Button {
                weight: 23,
                height: 32,
                label: String::from("Button2"),
        });

        s.push(b);
        s.push(b2);
        s.display();

        s.pop();
        s.display();

        let s = Box::new(oop_gui::SelectBox {
                weight: 23,
                height: 32,
                option: String::from("Select"),
        });
        s.push(s);
        s.display();
*/   // !! error for various type of push and display, maybe failed in the Vec specification

        let s = oop_gui::Screen {
                components: vec![
                        
                        Box::new(oop_gui::Button {
                                weight: 33,
                                height: 44,
                                label: String::from("Button"),
                        }),
                        Box::new(oop_gui::Button {
                                weight: 23,
                                height: 32,
                                label: String::from("Button2"),
                        }),
                        Box::new(oop_gui::SelectBox {
                                weight: 23,
                                height: 32,
                                option: String::from("Select"),
                        }),
                ],
        };

        s.display();
}

// dynamic polymorphism -- trait system(trait object)
// static polymorphism -- template system


// A trait is object safe if all the methods defined in the trait have the following properties:
//      1. The return type isn’t Self.  // !! unkown trait object
//      2. There are no generic type parameters. // !! will involve static polymorphism

