#![forbid(unknown_lints)]

use binds::{BindCount, BindsInternal, CollectBinds};
use row_locking::RowLocking;
use std::fmt;
use std::fmt::Write;
use std::marker::PhantomData;
use write_sql::WriteSql;

#[cfg(test)]
mod test;

mod binds;
mod expr;
mod filter;
mod from;
mod group;
mod join;
mod limit;
mod macros;
mod offset;
mod order;
mod query_dsl;
mod row_locking;
mod select;
mod write_sql;

pub mod sql_types;

pub use binds::{Bind, Binds};
pub use expr::{BinOp, Expr, ExprDsl, IntoExpr};
pub use filter::Filter;
pub use from::{from, FromClause, IntoSubQuery, SubQuery};
pub use group::Group;
pub use join::{Join, JoinKind, JoinOn, JoinOnDsl};
pub use limit::Limit;
pub use offset::Offset;
pub use order::{NullsPosition, NullsPositionDsl, Order, OrderDsl};
pub use query_dsl::QueryDsl;
pub use select::{count, star, Select, Selection};

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
    from: FromClause<T>,
    joins: Vec<Join>,
    filter: Option<Filter>,
    group: Option<Group>,
    having: Option<Filter>,
    order: Option<Order>,
    limit: Option<Limit>,
    offset: Option<Offset>,
    row_locking: RowLocking,
    _marker: PhantomData<T>,
}

impl<T> Query<T> {
    pub fn cast_to<K>(self) -> Query<K> {
        let Query {
            from,
            joins,
            filter,
            group,
            having,
            order,
            limit,
            offset,
            row_locking,
            _marker,
        } = self;

        Query {
            from: from.cast_to::<K>(),
            joins,
            filter,
            group,
            having,
            order,
            limit,
            offset,
            row_locking,
            _marker: PhantomData,
        }
    }

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

    fn add_join(&mut self, join: JoinOn, kind: JoinKind) {
        match join {
            JoinOn::Known { table, filter } => {
                self.joins.push(Join::Known {
                    kind,
                    table,
                    filter,
                });
            }
            JoinOn::Raw(sql) => {
                self.joins.push(Join::RawWithKind(sql));
            }
        }
    }
}

impl<T, K> From<K> for Query<T>
where
    K: Into<FromClause<T>>,
{
    fn from(from: K) -> Self {
        Self {
            from: from.into(),
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

#[derive(Debug, Clone)]
pub struct QueryWithSelect<T> {
    query: Query<T>,
    selection: Select,
}

impl<T> QueryWithSelect<T> {
    pub fn to_sql(self) -> (String, Binds) {
        let mut bind_count = BindCount::new();
        let mut sql = String::new();
        self.to_sql_string(&mut sql, &mut bind_count);
        let binds = self.collect_binds(&mut bind_count);
        (sql, binds)
    }

    fn to_sql_string<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) {
        let result = (|| -> fmt::Result {
            write!(f, "SELECT ")?;
            self.selection.write_sql(f, bind_count)?;

            write!(f, " FROM ")?;
            self.query.from.write_sql(f, bind_count)?;

            for join in &self.query.joins {
                write!(f, " ")?;
                join.write_sql(f, bind_count)?;
            }

            if let Some(filter) = &self.query.filter {
                write!(f, " WHERE ")?;
                filter.write_sql(f, bind_count)?;
            }

            if let Some(group) = &self.query.group {
                write!(f, " GROUP BY ")?;
                group.write_sql(f, bind_count)?;
            }

            if let Some(having) = &self.query.having {
                write!(f, " HAVING ")?;
                having.write_sql(f, bind_count)?;
            }

            if let Some(order) = &self.query.order {
                write!(f, " ORDER BY ")?;
                order.write_sql(f, bind_count)?;
            }

            if let Some(limit) = &self.query.limit {
                write!(f, " LIMIT ")?;
                limit.write_sql(f, bind_count)?;
            }

            if let Some(offset) = &self.query.offset {
                write!(f, " OFFSET ")?;
                offset.write_sql(f, bind_count)?;
            }

            self.query.row_locking.write_sql(f, bind_count)?;

            Ok(())
        })();

        result.expect("WriteSql should never fail");
    }

    fn collect_binds(&self, bind_count: &mut BindCount) -> Binds {
        let mut binds = BindsInternal::with_capacity(bind_count.count());
        self.query.collect_binds(&mut binds);
        Binds::from(binds)
    }

    pub fn cast_to<K>(self) -> QueryWithSelect<K> {
        let QueryWithSelect { query, selection } = self;

        QueryWithSelect {
            query: query.cast_to::<K>(),
            selection,
        }
    }
}

impl<T> WriteSql for QueryWithSelect<T> {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        self.to_sql_string(f, bind_count);
        Ok(())
    }
}

impl<T> CollectBinds for Box<QueryWithSelect<T>> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        self.query.collect_binds(binds)
    }
}

impl<T> CollectBinds for Query<T> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        self.from.collect_binds(binds);
        self.joins.collect_binds(binds);

        if let Some(filter) = &self.filter {
            filter.collect_binds(binds);
        }

        if let Some(group) = &self.group {
            group.collect_binds(binds);
        }

        if let Some(having) = &self.having {
            having.collect_binds(binds);
        }

        if let Some(order) = &self.order {
            order.collect_binds(binds);
        }

        if let Some(limit) = &self.limit {
            limit.collect_binds(binds);
        }

        if let Some(offset) = &self.offset {
            offset.collect_binds(binds);
        }

        self.row_locking.collect_binds(binds);
    }
}

impl CollectBinds for Table {
    fn collect_binds(&self, _: &mut BindsInternal) {}
}

impl CollectBinds for Vec<Join> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        for join in self {
            join.collect_binds(binds)
        }
    }
}
