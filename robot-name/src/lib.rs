use std::cell::RefCell;

thread_local! {
    static ROBOTS: RefCell<Vec<String>> = {
        let mut v = Vec::with_capacity(26*26*1000);
        for ch1 in 'A'..='Z' {
            for ch2 in 'A'..='Z' {
                for ch3 in '0'..='9' {
                    for ch4 in '0'..='9' {
                        for ch5 in '0'..='9' {
                            let name = format!("{}{}{}{}{}", ch1,ch2,ch3,ch4,ch5);
                            v.push(name);
                        }
                    }
                }
            }
        }
        RefCell::new(v)
    };
}
#[derive(Debug)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: next_available(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = next_available();
    }
}
fn next_available() -> String {
    ROBOTS
        .with(|names| names.borrow_mut().pop())
        .expect("no more name combinations available")
}

//if the Robot is destroyed -> make its name available for the new one
impl Drop for Robot {
    fn drop(&mut self) {
        ROBOTS.with(|names| names.borrow_mut().push(self.name.clone()))
    }
}
