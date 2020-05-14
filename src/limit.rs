use crate::binds::{Bind, BindCount, BindsInternal, CollectBinds};
use crate::WriteSql;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Limit {
    Count(i32),
    Raw(String),
}

impl Limit {
    pub fn raw(sql: &str) -> Self {
        Self::Raw(sql.to_string())
    }
}

impl<T> From<T> for Limit
where
    T: Into<i32>,
{
    fn from(count: T) -> Self {
        Limit::Count(count.into())
    }
}

impl WriteSql for &Limit {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Limit::Count(_) => bind_count.write_sql(f),
            Limit::Raw(sql) => write!(f, "{}", sql),
        }
    }
}

impl CollectBinds for Limit {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            Limit::Count(count) => {
                binds.push(Bind::I32(*count));
            }
            Limit::Raw(_) => {}
        }
    }
}
