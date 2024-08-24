use std::{cell::RefCell, rc::Rc};

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You're over your quote.");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You're used up over 90% of your quota.");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You're used up over 75% of your quota.");
        }
    }
}

pub fn refcell_borrow_mut() {
    let rc1 = Rc::new(RefCell::new(10));
    let rc2 = rc1.clone();
    let rc3 = rc1.clone();

    *rc3.borrow_mut() = 20;
    println!("val: {}", rc1.borrow());

    rc3.replace(30);
    println!("val: {}", rc2.borrow());
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn it_sends_an_over_75_percentage_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
