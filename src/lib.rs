#![forbid(unknown_lints)]

use crate::binds::{BindCount, BindsInternal, CollectBinds};
use std::fmt;
use std::fmt::Write;

mod binds;
mod expr;
mod filter;
mod grouping;
mod macros;
mod ordering;
mod query_dsl;
mod selection;
mod join;

pub mod sql_types;

pub use binds::{Bind, Binds};
pub use expr::{BinOp, Expr, ExprDsl, IntoExpr};
pub use filter::Filter;
pub use grouping::Grouping;
pub use ordering::{Ordering, OrderingDsl};
pub use query_dsl::QueryDsl;
pub use selection::Selection;
pub use join::{Join, JoinKind, PartialJoin, JoinOnDsl};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Table {
    name: &'static str,
}

impl WriteSql for Table {
    fn write_sql<W: Write>(&self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        write!(f, "\"{}\"", self.name)
    }
}

impl Table {
    pub fn new(name: &'static str) -> Self {
        Table { name }
    }

    pub fn name(&self) -> &'static str {
        &self.name
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Column {
    table: &'static str,
    name: &'static str,
}

impl Column {
    pub fn new(table: &'static str, name: &'static str) -> Self {
        Self { table, name }
    }
}

impl WriteSql for Column {
    fn write_sql<W: Write>(&self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        write!(f, "\"{}\".\"{}\"", self.table, self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    table: Table,
    joins: Vec<Join>,
    filter: Option<Filter>,
    group: Option<Grouping>,
    having: Option<Filter>,
    order: Option<Ordering>,
    limit: Option<u64>,
}

impl Query {
    pub fn remove_joins(mut self) -> Self {
        self.joins.clear();
        self
    }

    pub fn remove_filters(mut self) -> Self {
        self.filter = None;
        self
    }

    pub fn remove_group_by(mut self) -> Self {
        self.group = None;
        self
    }

    pub fn remove_having(mut self) -> Self {
        self.having = None;
        self
    }

    pub fn remove_order_by(mut self) -> Self {
        self.order = None;
        self
    }

    pub fn remove_limit(mut self) -> Self {
        self.limit = None;
        self
    }

    fn to_sql(&mut self, selection: Selection) -> (String, Binds) {
        let mut f = String::new();

        let mut bind_count = BindCount::new();

        let result = (|| -> fmt::Result {
            write!(f, "SELECT ")?;
            selection.write_sql(&mut f, &mut bind_count)?;

            write!(f, " FROM ")?;
            self.table.write_sql(&mut f, &mut bind_count)?;

            for join in &self.joins {
                write!(f, " ")?;
                join.write_sql(&mut f, &mut bind_count)?;
            }

            if let Some(filter) = &self.filter {
                write!(f, " WHERE ")?;
                filter.write_sql(&mut f, &mut bind_count)?;
            }

            if let Some(group) = &self.group {
                write!(f, " GROUP BY ")?;
                group.write_sql(&mut f, &mut bind_count)?;
            }

            if let Some(having) = &self.having {
                write!(f, " HAVING ")?;
                having.write_sql(&mut f, &mut bind_count)?;
            }

            if let Some(order) = &self.order {
                write!(f, " ORDER BY ")?;
                order.write_sql(&mut f, &mut bind_count)?;
            }

            if let Some(_) = &self.limit {
                write!(f, " LIMIT ")?;
                bind_count.write_sql(&mut f)?;
            }

            Ok(())
        })();

        let mut binds = BindsInternal::with_capacity(bind_count.count());
        self.collect_binds(&mut binds);

        result.expect("WriteSql should never fail");

        (f, Binds::from(binds))
    }
}

impl From<Table> for Query {
    fn from(table: Table) -> Self {
        Self {
            table: table.into(),
            filter: None,
            joins: Vec::new(),
            group: None,
            having: None,
            order: None,
            limit: None,
        }
    }
}

trait WriteSql {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result;
}

#[cfg(test)]
mod test;
