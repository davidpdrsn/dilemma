use crate::binds::{Bind, BindCount, BindsInternal, CollectBinds};
use crate::WriteSql;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub struct Offset(pub(crate) OffsetI);

#[derive(Debug, Clone)]
pub(crate) enum OffsetI {
    Count(i32),
    Raw(String),
}

impl Offset {
    pub fn raw(sql: &str) -> Self {
        OffsetI::Raw(sql.to_string()).into()
    }
}

impl Into<Offset> for OffsetI {
    fn into(self) -> Offset {
        Offset(self)
    }
}

impl<T> From<T> for Offset
where
    T: Into<i32>,
{
    fn from(count: T) -> Self {
        OffsetI::Count(count.into()).into()
    }
}

impl WriteSql for &OffsetI {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            OffsetI::Count(_) => bind_count.write_sql(f),
            OffsetI::Raw(sql) => write!(f, "{}", sql),
        }
    }
}

impl CollectBinds for OffsetI {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            OffsetI::Count(count) => {
                binds.push(Bind::I32(*count));
            }
            OffsetI::Raw(_) => {}
        }
    }
}
