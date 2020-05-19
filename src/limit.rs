use crate::binds::{Bind, BindCount, BindsInternal, CollectBinds};
use crate::WriteSql;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub struct Limit(pub(crate) LimitI);

impl Into<Limit> for LimitI {
    fn into(self) -> Limit {
        Limit(self)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum LimitI {
    Count(i32),
    Raw(String),
}

impl Limit {
    pub fn raw(sql: &str) -> Self {
        LimitI::Raw(sql.to_string()).into()
    }
}

impl<T> From<T> for Limit
where
    T: Into<i32>,
{
    fn from(count: T) -> Self {
        LimitI::Count(count.into()).into()
    }
}

impl WriteSql for &LimitI {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            LimitI::Count(_) => bind_count.write_sql(f),
            LimitI::Raw(sql) => write!(f, "{}", sql),
        }
    }
}

impl CollectBinds for LimitI {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            LimitI::Count(count) => {
                binds.push(Bind::I32(*count));
            }
            LimitI::Raw(_) => {}
        }
    }
}
