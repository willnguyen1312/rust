use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq)]
pub struct CustomSet<T: Clone + Eq + Hash> {
    map: HashMap<T, ()>,
}

impl<T: Clone + Eq + Hash> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = Self {
            map: HashMap::new(),
        };
        for key in input {
            set.add(key.clone());
        }
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.map.contains_key(element)
    }

    pub fn add(&mut self, element: T) {
        self.map.insert(element, ());
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.map.keys().all(|key| other.contains(key))
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.map.keys().any(|key| other.contains(key))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self {
            map: HashMap::new(),
        };

        for key in self.map.keys() {
            if other.contains(key) {
                result.add(key.clone())
            }
        }

        result
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for key in other.map.keys() {
            result.map.remove(&key);
        }

        result
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for key in other.map.keys() {
            if !result.contains(key) {
                result.add(key.clone())
            }
        }

        result
    }
}
