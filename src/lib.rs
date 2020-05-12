#![forbid(unknown_lints)]

use binds::{BindCount, BindsInternal, CollectBinds};
use row_locking::RowLocking;
use std::fmt;
use std::fmt::Write;
use std::marker::PhantomData;
use write_sql::WriteSql;

mod binds;
mod expr;
mod filter;
mod grouping;
mod join;
mod macros;
mod ordering;
mod query_dsl;
mod row_locking;
mod selection;
mod write_sql;

pub mod sql_types;

pub use binds::{Bind, Binds};
pub use expr::{BinOp, Expr, ExprDsl, IntoExpr};
pub use filter::Filter;
pub use grouping::Grouping;
pub use join::{Join, JoinKind, JoinOnDsl, PartialJoin};
pub use ordering::{Ordering, OrderingDsl};
pub use query_dsl::QueryDsl;
pub use selection::Selection;

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
pub struct Query<T> {
    table: Table,
    joins: Vec<Join>,
    filter: Option<Filter>,
    group: Option<Grouping>,
    having: Option<Filter>,
    order: Option<Ordering>,
    limit: Option<u64>,
    offset: Option<u64>,
    row_locking: RowLocking,
    _marker: PhantomData<T>,
}

impl<T> Query<T> {
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

    pub fn remove_offset(mut self) -> Self {
        self.offset = None;
        self
    }

    pub fn remove_for_update(mut self) -> Self {
        self.row_locking.for_update = false;
        self
    }

    pub fn remove_skip_locked(mut self) -> Self {
        self.row_locking.skip_locked = false;
        self
    }

    pub fn remove_for_key_share(mut self) -> Self {
        self.row_locking.for_key_share = false;
        self
    }

    pub fn remove_for_no_key_update(mut self) -> Self {
        self.row_locking.for_no_key_update = false;
        self
    }

    pub fn remove_for_share(mut self) -> Self {
        self.row_locking.for_share = false;
        self
    }

    pub fn remove_no_wait(mut self) -> Self {
        self.row_locking.no_wait = false;
        self
    }
}

impl<T> From<Table> for Query<T> {
    fn from(table: Table) -> Self {
        Self {
            table: table.into(),
            filter: None,
            joins: Vec::new(),
            group: None,
            having: None,
            order: None,
            limit: None,
            offset: None,
            row_locking: RowLocking::new(),
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct QueryWithSelection<T> {
    query: Query<T>,
    selection: Selection,
}

impl<T> QueryWithSelection<T> {
    pub fn to_sql(self) -> (String, Binds) {
        let mut bind_count = BindCount::new();
        let sql = self.to_sql_string(&mut bind_count);
        let binds = self.collect_binds(&mut bind_count);
        (sql, binds)
    }

    fn to_sql_string(&self, bind_count: &mut BindCount) -> String {
        let mut f = String::new();

        let result = (|| -> fmt::Result {
            write!(f, "SELECT ")?;
            self.selection.write_sql(&mut f, bind_count)?;

            write!(f, " FROM ")?;
            self.query.table.write_sql(&mut f, bind_count)?;

            for join in &self.query.joins {
                write!(f, " ")?;
                join.write_sql(&mut f, bind_count)?;
            }

            if let Some(filter) = &self.query.filter {
                write!(f, " WHERE ")?;
                filter.write_sql(&mut f, bind_count)?;
            }

            if let Some(group) = &self.query.group {
                write!(f, " GROUP BY ")?;
                group.write_sql(&mut f, bind_count)?;
            }

            if let Some(having) = &self.query.having {
                write!(f, " HAVING ")?;
                having.write_sql(&mut f, bind_count)?;
            }

            if let Some(order) = &self.query.order {
                write!(f, " ORDER BY ")?;
                order.write_sql(&mut f, bind_count)?;
            }

            if let Some(_) = &self.query.limit {
                write!(f, " LIMIT ")?;
                bind_count.write_sql(&mut f)?;
            }

            if let Some(_) = &self.query.offset {
                write!(f, " OFFSET ")?;
                bind_count.write_sql(&mut f)?;
            }

            self.query.row_locking.write_sql(&mut f, bind_count)?;

            Ok(())
        })();

        result.expect("WriteSql should never fail");

        f
    }

    fn collect_binds(&self, bind_count: &mut BindCount) -> Binds {
        let mut binds = BindsInternal::with_capacity(bind_count.count());
        self.query.collect_binds(&mut binds);
        Binds::from(binds)
    }
}

#[cfg(test)]
mod test;
