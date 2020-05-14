use crate::binds::{Bind, BindCount, BindsInternal, CollectBinds};
use crate::WriteSql;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Offset {
    Count(i32),
    Raw(String),
}

impl Offset {
    pub fn raw(sql: &str) -> Self {
        Self::Raw(sql.to_string())
    }
}

impl<T> From<T> for Offset
where
    T: Into<i32>,
{
    fn from(count: T) -> Self {
        Offset::Count(count.into())
    }
}

impl WriteSql for &Offset {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Offset::Count(_) => bind_count.write_sql(f),
            Offset::Raw(sql) => write!(f, "{}", sql),
        }
    }
}

impl CollectBinds for Offset {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            Offset::Count(count) => {
                binds.push(Bind::I32(*count));
            }
            Offset::Raw(_) => {}
        }
    }
}
