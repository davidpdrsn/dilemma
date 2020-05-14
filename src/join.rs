use crate::binds::BindCount;
use crate::binds::{BindsInternal, CollectBinds};
use crate::{filter::Filter, Table, WriteSql};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Join {
    Known {
        kind: JoinKind,
        table: Table,
        filter: Filter,
    },
    RawWithKind(String),
    Raw(String),
}

impl WriteSql for &Join {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Join::Known {
                kind,
                table,
                filter,
            } => {
                kind.write_sql(f, bind_count)?;
                table.write_sql(f, bind_count)?;
                write!(f, " ON ")?;
                filter.write_sql(f, bind_count)?;
            }
            Join::RawWithKind(sql) => {
                write!(f, "{}", sql)?;
            }
            Join::Raw(sql) => {
                write!(f, "{}", sql)?;
            }
        }

        Ok(())
    }
}

impl CollectBinds for Join {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            Join::Known {
                kind: _,
                table,
                filter,
            } => {
                table.collect_binds(binds);
                filter.collect_binds(binds);
            }
            Join::RawWithKind(_) => {}
            Join::Raw(_) => {}
        }
    }
}

impl Join {
    pub fn raw(sql: &str) -> JoinOn {
        JoinOn::Raw(sql.to_string())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum JoinKind {
    Default,
    Inner,
    Outer,
}

impl WriteSql for &JoinKind {
    fn write_sql<W: Write>(self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        match self {
            JoinKind::Default => write!(f, "JOIN ")?,
            JoinKind::Inner => write!(f, "INNER JOIN ")?,
            JoinKind::Outer => write!(f, "OUTER JOIN ")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum JoinOn {
    Known { table: Table, filter: Filter },
    Raw(String),
}

pub trait JoinOnDsl {
    fn on(self, filter: Filter) -> JoinOn;
}

impl<T> JoinOnDsl for T
where
    T: Into<Table>,
{
    fn on(self, filter: Filter) -> JoinOn {
        JoinOn::Known {
            table: self.into(),
            filter,
        }
    }
}
