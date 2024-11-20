use std::collections::BTreeMap;

#[derive(Default)]
pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self
            .grades
            .values()
            .flatten()
            .all(|already_added_student| already_added_student.ne(student))
        {
            self.grades
                .entry(grade)
                .or_default()
                .push(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut vec = self.grades.get(&grade).cloned().unwrap_or_default();
        vec.sort();
        vec
    }
}
