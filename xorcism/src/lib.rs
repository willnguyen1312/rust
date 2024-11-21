use std::{
    borrow::Borrow,
    io::{Read, Write},
};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    current_byte_index: usize,
}

pub trait Captures<'a> {}
impl<'a, T> Captures<'a> for T {}

struct Reader<'a, T: Read> {
    pub inner: T,
    pub xor: Xorcism<'a>,
}

impl<'a, T: Read> Read for Reader<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.inner.read(buf)?;
        self.xor.munge_in_place(buf);
        Ok(len)
    }
}

struct Writer<'a, T: Write> {
    pub inner: T,
    pub xor: Xorcism<'a>,
}

impl<'a, T: Write> Write for Writer<'a, T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let xored_buf: Vec<_> = self.xor.munge(buf).collect();
        let len = self.inner.write(&xored_buf)?;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl<'a> Xorcism<'a> {
    pub fn new<T: AsRef<[u8]> + ?Sized>(key: &'a T) -> Xorcism<'a> {
        Xorcism {
            key: key.as_ref(),
            current_byte_index: 0,
        }
    }
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .for_each(|byte| *byte = *byte ^ self.next_byte());
    }

    pub fn munge<'b, T, U>(&'b mut self, data: T) -> impl Iterator<Item = u8> + 'b + Captures<'a>
    where
        T: IntoIterator<Item = U> + 'b,
        U: Borrow<u8>,
    {
        data.into_iter().map(move |x| x.borrow() ^ self.next_byte())
    }

    fn next_byte(&mut self) -> u8 {
        let current_byte_index = self.current_byte_index;
        self.current_byte_index = (self.current_byte_index + 1) % self.key.len();
        self.key[current_byte_index]
    }

    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        Reader {
            inner: reader,
            xor: self,
        }
    }

    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        Writer {
            inner: writer,
            xor: self,
        }
    }
}
