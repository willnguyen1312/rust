use std::collections::VecDeque;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    field: VecDeque<T>,
    capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            field: VecDeque::<T>::with_capacity(capacity),
            capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.field.len() == self.capacity {
            Err(Error::FullBuffer)
        } else {
            self.field.push_back(element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.field.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            Ok(self.field.pop_front().unwrap())
        }
    }

    pub fn clear(&mut self) {
        self.field.clear()
    }

    pub fn overwrite(&mut self, element: T) {
        if self.field.len() == self.capacity {
            self.field.pop_front();
        }
        self.field.push_back(element);
    }
}
