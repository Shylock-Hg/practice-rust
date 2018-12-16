pub trait Messenger {
        fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
}

impl<'a, T> LimitTracker<'a, T>
        where T: Messenger {

        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
                LimitTracker {
                        messenger,
                        value: 0,
                        max,
                }
        }

        pub fn set_value(&mut self, value: usize) {
                self.value = value;

                let percentage_of_max = self.value as f64 / self.max as f64;
                if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
                        self.messenger.send("Warning: You've used up over 75% of your quora!");
                } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
                        self.messenger.send("Urgent warning: You've used up over 90% of yopur quota!");
                } else {
                        self.messenger.send("Error: You are over your quota!");
                }
        }
}

// RefCell<T>::borrow_mut() return RefMut<T>
// RefCell<T>::borrow() return Ref<T>
// RefCell track the RefMUT<t> and Ref<T> in runtime, and allow many immutable  borrows or one mutable borrow at any point in time.
// error usage will call panic at runtime.


#[cfg(test)]
mod tests {
        use super::*;
        use std::cell::RefCell;

        struct MockMessenger {
                sent_messages: RefCell<Vec<String>>,  // interior mutability
        }

        impl MockMessenger {
                fn new() -> MockMessenger {
                        MockMessenger {sent_messages: RefCell::new(vec![])}
                }
        }

        impl Messenger for MockMessenger {
                fn send(&self, message: &str) {  // interior mutability
                        self.sent_messages.borrow_mut().push(String::from(message));

                        // !!  will panic
                        // let mut one = self.sent_messages.borrow_mut();
                        // let mut two = self.sent_messages.borrow_mut();
                        // one.push(String::from(message));
                        // two.push(String::from(message));
                }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message(){
                let mock_messenger = MockMessenger::new();
                let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

                limit_tracker.set_value(80);
                assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
}
