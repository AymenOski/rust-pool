use std::rc::Rc;
use std::cell::RefCell;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: RefCell::new(Vec::new()),
            value: RefCell::new(0),
            max,
        }
    }

     pub fn set_value(&self, rc: &Rc<i32>) {
        let count = Rc::strong_count(rc);
        let percentage = (count * 100) / self.max;

        if count > self.max {
            // Error if count exceeds max
            self.messages.borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else {
            self.value.replace(count);

            if percentage > 70 {
                self.messages.borrow_mut()
                    .push(format!(
                        "Warning: You have used up over {}% of your quota!",
                        percentage
                    ));
            }
        }
    }

    pub fn peek(&self, rc: &Rc<i32>) {
        let count = Rc::strong_count(rc);
        let percentage = (count * 100) / self.max;

        self.messages.borrow_mut()
            .push(format!(
                "Info: This value would use {}% of your quota",
                percentage
            ));
    }

}

/*
    * Q & A :
    * Q1 : Why do we use `Rc` and `RefCell` here?
    - A1 : We use `Rc` (Reference Counted) to enable multiple ownership of the same data, allowing us to have multiple references to the same value. `RefCell` is used to allow interior mutability, which means we can mutate the data inside an `Rc` even when we have multiple references to it. This combination allows us to track the number of references to
*/