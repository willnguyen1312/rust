use std::ops::Rem;

pub struct Matcher<'a, T> {
    matcher: Box<dyn Fn(T) -> bool + 'a>,
    subs: &'a str,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new(matcher: impl Fn(T) -> bool + 'a, subs: &'a str) -> Matcher<T> {
        Self {
            matcher: Box::new(matcher),
            subs,
        }
    }
}

pub struct Fizzy<'a, T> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T> Fizzy<'a, T> {
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I>(&'a self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        T: Clone + Copy + ToString + 'a,
        I: Iterator<Item = T> + 'a,
    {
        iter.map(move |x| {
            let fz: String = self
                .matchers
                .iter()
                .filter_map(|m| (m.matcher)(x).then(|| m.subs))
                .collect();
            if fz.is_empty() {
                x.to_string()
            } else {
                fz
            }
        })
    }
}

pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: Clone + Copy + Default + ToString + Rem<Output = T> + PartialEq + From<u8>,
{
    Fizzy::<T>::new()
        .add_matcher(Matcher::new(
            |n: T| n.rem(T::from(3)) == T::default(),
            "fizz",
        ))
        .add_matcher(Matcher::new(
            |n: T| n.rem(T::from(5)) == T::default(),
            "buzz",
        ))
}
