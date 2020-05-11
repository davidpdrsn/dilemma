use crate::binds::BindCount;
use std::fmt::{self, Write};

pub trait WriteSql {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result;
}
