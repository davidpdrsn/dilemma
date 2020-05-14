use crate::binds::{BindCount, BindsInternal, CollectBinds};
use crate::{write_sql::WriteSql, Column};
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Distinct {
    EachRow,
    On(Vec<Column>),
}

impl WriteSql for Distinct {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Distinct::EachRow => write!(f, "DISTINCT "),
            Distinct::On(cols) => {
                write!(f, "DISTINCT ON (")?;
                cols.write_sql(f, bind_count)?;
                write!(f, ") ")?;
                Ok(())
            }
        }
    }
}

impl CollectBinds for Distinct {
    fn collect_binds(&self, _: &mut BindsInternal) {}
}
