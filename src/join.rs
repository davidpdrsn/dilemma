use crate::binds::BindCount;
use crate::{filter::Filter, Table, WriteSql};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Join {
    pub(crate) kind: JoinKind,
    pub(crate) table: Table,
    pub(crate) filter: Filter,
}

impl WriteSql for Join {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        self.kind.write_sql(f, bind_count)?;
        self.table.write_sql(f, bind_count)?;

        write!(f, " ON ")?;

        self.filter.write_sql(f, bind_count)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum JoinKind {
    Inner,
    Outer,
}

impl WriteSql for JoinKind {
    fn write_sql<W: Write>(&self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        match self {
            JoinKind::Inner => write!(f, "INNER JOIN ")?,
            JoinKind::Outer => write!(f, "OUTER JOIN ")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PartialJoin {
    pub(crate) table: Table,
    pub(crate) filter: Filter,
}

pub trait JoinOnDsl {
    fn on(self, filter: Filter) -> PartialJoin;
}

impl<T> JoinOnDsl for T
where
    T: Into<Table>,
{
    fn on(self, filter: Filter) -> PartialJoin {
        PartialJoin {
            table: self.into(),
            filter,
        }
    }
}
