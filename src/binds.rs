use std::fmt::{self, Write};
use std::iter::IntoIterator;
use std::vec::IntoIter;

pub struct BindCount(usize);

impl BindCount {
    pub fn new() -> Self {
        Self(1)
    }

    fn next(&mut self) -> usize {
        let count = self.0;
        self.0 += 1;
        count
    }

    pub fn count(&self) -> usize {
        self.0
    }

    pub fn write_sql<W: Write>(&mut self, f: &mut W) -> fmt::Result {
        write!(f, "${}", self.next())
    }
}

pub trait CollectBinds {
    fn collect_binds(&self, binds: &mut BindsInternal);
}

#[derive(Debug)]
pub struct BindsInternal {
    vec: Vec<Bind>,
}

impl BindsInternal {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, bind: Bind) {
        self.vec.push(bind);
    }
}

#[derive(Debug)]
pub struct Binds {
    iter: IntoIter<Bind>,
}

impl Iterator for Binds {
    type Item = Bind;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl From<BindsInternal> for Binds {
    fn from(internal: BindsInternal) -> Self {
        Binds {
            iter: internal.vec.into_iter(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Bind {
    String(String),
    I32(i32),
    U64(u64),
}
