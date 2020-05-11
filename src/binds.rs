use crate::{Expr, Filter, Join, Query, Table};
use std::vec::IntoIter;
use std::fmt::{self, Write};
use std::iter::IntoIterator;

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

    fn push(&mut self, bind: Bind) {
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
        Binds { iter: internal.vec.into_iter() }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Bind {
    String(String),
    I32(i32),
    U64(u64),
}

impl CollectBinds for Query {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        self.table.collect_binds(binds);
        self.joins.collect_binds(binds);

        if let Some(filter) = &self.filter {
            filter.collect_binds(binds);
        }

        if let Some(limit) = &self.limit {
            binds.push(Bind::U64(*limit));
        }
    }
}

impl CollectBinds for Table {
    fn collect_binds(&self, _: &mut BindsInternal) {}
}

impl CollectBinds for Vec<Join> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        for join in self {
            join.collect_binds(binds)
        }
    }
}

impl CollectBinds for Join {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        self.table.collect_binds(binds);
        self.filter.collect_binds(binds);
    }
}

impl CollectBinds for Filter {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            Filter::Op { lhs, op: _, rhs } => {
                lhs.collect_binds(binds);
                rhs.collect_binds(binds);
            }
            Filter::And(lhs, rhs) => {
                lhs.collect_binds(binds);
                rhs.collect_binds(binds);
            }
            Filter::Or(lhs, rhs) => {
                lhs.collect_binds(binds);
                rhs.collect_binds(binds);
            }
        }
    }
}

impl CollectBinds for Expr {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            Expr::Column(_) => {}
            Expr::I32(value) => binds.push(Bind::I32(*value)),
            Expr::String(value) => binds.push(Bind::String(value.clone())),
        }
    }
}
