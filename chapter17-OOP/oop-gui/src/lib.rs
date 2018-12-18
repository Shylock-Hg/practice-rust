// the trait object must implement by pointer
// the trait can't define data field

pub trait Draw {
        fn draw(&self);
}


// to describe the GUI components by linear container
// !! better to represent by tree and draw follow BFS
pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,  // means any Box type implement Draw trait
}

impl Screen {
        pub fn display(&self) {
                for comp in self.components.iter() {
                        comp.draw();
                }
        }

        pub fn push(&mut self, comp: Box<dyn Draw>) {
                self.components.push(comp);
        }

        pub fn pop(&mut self) -> Option<Box<dyn Draw>> {
                self.components.pop()
        }
}

// conponents

// button
pub struct Button {
        pub weight: u32,
        pub height: u32,
        pub label: String,
}

impl Draw for Button {
        fn draw(&self) {
                println!("Draw button with weight {}, height {} and label {}!",
                        self.weight, self.height, self.label);
        }
}

// select box
pub struct SelectBox {
        pub weight: u32,
        pub height: u32,
        pub option: String,
}

impl Draw for SelectBox {
        fn draw(&self) {
                println!("Draw selecet box with weight {}, height {} and option {}!",
                        self.weight, self.height, self.option);
        }
}
