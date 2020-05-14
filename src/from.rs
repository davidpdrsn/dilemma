use crate::binds::{BindCount, BindsInternal, CollectBinds};
use crate::write_sql::WriteSql;
use crate::Table;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum FromClause {
    Table(Table),
}

impl From<Table> for FromClause {
    fn from(table: Table) -> Self {
        FromClause::Table(table)
    }
}

impl WriteSql for FromClause {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            FromClause::Table(table) => table.write_sql(f, bind_count),
        }
    }
}

impl CollectBinds for FromClause {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            FromClause::Table(table) => table.collect_binds(binds),
        }
    }
}
