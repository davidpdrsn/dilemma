#![forbid(unknown_lints)]

use binds::{BindCount, BindsInternal, CollectBinds};
use join::CastVecJoin;
use row_locking::RowLocking;
use std::fmt;
use std::fmt::Write;
use std::marker::PhantomData;
use write_sql::WriteSql;

mod macros;

#[cfg(test)]
mod test;

mod binds;
mod cte;
mod distinct;
mod expr;
mod filter;
mod from;
mod group;
mod join;
mod limit;
mod offset;
mod order;
mod query_dsl;
mod row_locking;
mod select;
mod write_sql;

pub mod sql_types;

pub use binds::{Bind, Binds};
pub use cte::Ctes;
pub use distinct::Distinct;
pub use expr::{BinOp, Expr, ExprDsl, IntoExpr, UnOp};
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

impl WriteSql for &Table {
    fn write_sql<W: Write>(self, f: &mut W, _: &mut BindCount) -> fmt::Result {
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

impl WriteSql for &Column {
    fn write_sql<W: Write>(self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        write!(f, "\"{}\".\"{}\"", self.table, self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Query<T> {
    ctes: Ctes<T>,
    from: FromClause<T>,
    joins: Vec<Join<T>>,
    filter: Option<Filter>,
    group: Option<Group>,
    having: Option<Filter>,
    order: Option<Order>,
    limit: Option<Limit>,
    offset: Option<Offset>,
    row_locking: RowLocking,
    distinct: Option<Distinct>,
    explain: Option<Explain>,
    _marker: PhantomData<T>,
}

impl<T> Query<T> {
    pub fn cast_to<K>(self) -> Query<K> {
        let Query {
            from,
            ctes,
            joins,
            filter,
            group,
            having,
            order,
            limit,
            offset,
            row_locking,
            distinct,
            explain,
            _marker,
        } = self;

        Query {
            from: from.cast_to::<K>(),
            ctes: ctes.cast_to::<K>(),
            joins: joins.cast_to::<K>(),
            filter,
            group,
            having,
            order,
            limit,
            offset,
            row_locking,
            distinct,
            explain,
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

    pub fn remove_distinct(mut self) -> Self {
        self.distinct = None;
        self
    }

    pub fn remove_ctes(mut self) -> Self {
        self.ctes = Ctes::default();
        self
    }

    pub fn remove_explain(mut self) -> Self {
        self.explain = None;
        self
    }

    fn add_join(&mut self, join: JoinOn<T>, kind: JoinKind) {
        match join {
            JoinOn::Known { from, filter } => {
                self.joins.push(Join::Known { kind, from, filter });
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
            ctes: Ctes::default(),
            filter: None,
            joins: Vec::new(),
            group: None,
            having: None,
            order: None,
            limit: None,
            offset: None,
            distinct: None,
            row_locking: RowLocking::new(),
            explain: None,
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

        let mut binds = BindsInternal::with_capacity(bind_count.count());
        self.collect_binds(&mut binds);
        (sql, Binds::from(binds))
    }

    fn to_sql_string<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) {
        let result = (|| -> fmt::Result {
            if let Some(explain) = self.query.explain {
                explain.write_sql(f, bind_count)?;
            }

            self.query.ctes.write_sql(f, bind_count)?;

            write!(f, "SELECT ")?;

            if let Some(distinct) = &self.query.distinct {
                distinct.write_sql(f, bind_count)?;
            }

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
                limit.0.write_sql(f, bind_count)?;
            }

            if let Some(offset) = &self.query.offset {
                write!(f, " OFFSET ")?;
                offset.0.write_sql(f, bind_count)?;
            }

            self.query.row_locking.write_sql(f, bind_count)?;

            Ok(())
        })();

        result.expect("WriteSql should never fail");
    }

    fn collect_binds(&self, binds: &mut BindsInternal) {
        self.query.collect_binds(binds);
    }

    pub fn cast_to<K>(self) -> QueryWithSelect<K> {
        let QueryWithSelect { query, selection } = self;

        QueryWithSelect {
            query: query.cast_to::<K>(),
            selection,
        }
    }

    pub fn union<K>(self, other: QueryWithSelect<K>) -> Union<T> {
        Union::Pair(UnionKind::Default, self, other.cast_to())
    }

    pub fn union_all<K>(self, other: QueryWithSelect<K>) -> Union<T> {
        Union::Pair(UnionKind::All, self, other.cast_to())
    }

    pub fn union_distinct<K>(self, other: QueryWithSelect<K>) -> Union<T> {
        Union::Pair(UnionKind::Distinct, self, other.cast_to())
    }
}

impl<T> WriteSql for &QueryWithSelect<T> {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
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
        self.ctes.collect_binds(binds);
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
            limit.0.collect_binds(binds);
        }

        if let Some(offset) = &self.offset {
            offset.0.collect_binds(binds);
        }

        self.row_locking.collect_binds(binds);
    }
}

impl CollectBinds for Table {
    fn collect_binds(&self, _: &mut BindsInternal) {}
}

pub trait IntoColumns {
    fn into_columns(self) -> Vec<Column>;
}

impl<T> IntoColumns for T
where
    T: Into<Column>,
{
    fn into_columns(self) -> Vec<Column> {
        vec![self.into()]
    }
}

impl<T> IntoColumns for (T,)
where
    T: Into<Column>,
{
    fn into_columns(self) -> Vec<Column> {
        vec![self.0.into()]
    }
}

macro_rules! impl_into_columns {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> IntoColumns for ($first, $second)
        where
            $first: Into<Column>,
            $second: Into<Column>,
        {
            fn into_columns(self) -> Vec<Column> {
                let ($first, $second) = self;
                vec![$first.into(), $second.into()]
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<$head, $($tail),*> IntoColumns for ($head, $($tail),*)
        where
            $head: Into<Column>,
            $( $tail: Into<Column> ),*
        {
            fn into_columns(self) -> Vec<Column> {
                let ($head, $($tail),*) = self;
                vec![
                    $head.into(),
                    $( $tail.into(), )*
                ]
            }
        }

        impl_into_columns!($($tail),*,);
    };
}

impl_into_columns!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);

#[derive(Debug)]
pub enum UnionKind {
    Default,
    All,
    Distinct,
}

#[derive(Debug)]
pub enum Union<T> {
    Pair(UnionKind, QueryWithSelect<T>, QueryWithSelect<T>),
    And(UnionKind, Box<Union<T>>, QueryWithSelect<T>),
}

impl WriteSql for &UnionKind {
    fn write_sql<W: Write>(self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        match self {
            UnionKind::Default => write!(f, " UNION "),
            UnionKind::All => write!(f, " UNION ALL "),
            UnionKind::Distinct => write!(f, " UNION DISTINCT "),
        }
    }
}

impl<T> Union<T> {
    pub fn to_sql(self) -> (String, Binds) {
        let mut bind_count = BindCount::new();
        let mut sql = String::new();

        self.to_sql_recurs(&mut sql, &mut bind_count);

        let mut binds = BindsInternal::with_capacity(bind_count.count());
        self.collect_binds_recurs(&mut binds);

        (sql, Binds::from(binds))
    }

    fn to_sql_recurs(&self, sql: &mut String, bind_count: &mut BindCount) {
        match self {
            Union::Pair(kind, lhs, rhs) => {
                lhs.to_sql_string(sql, bind_count);
                kind.write_sql(sql, bind_count).unwrap();
                rhs.to_sql_string(sql, bind_count);
            }
            Union::And(kind, head, tail) => {
                head.to_sql_recurs(sql, bind_count);
                kind.write_sql(sql, bind_count).unwrap();
                tail.to_sql_string(sql, bind_count);
            }
        }
    }

    fn collect_binds_recurs(&self, binds: &mut BindsInternal) {
        match self {
            Union::Pair(_, lhs, rhs) => {
                lhs.collect_binds(binds);
                rhs.collect_binds(binds);
            }
            Union::And(_, head, tail) => {
                head.collect_binds_recurs(binds);
                tail.collect_binds(binds);
            }
        }
    }

    pub fn union<K>(self, other: QueryWithSelect<K>) -> Union<T> {
        Union::And(UnionKind::Default, Box::new(self), other.cast_to())
    }

    pub fn union_all<K>(self, other: QueryWithSelect<K>) -> Union<T> {
        Union::And(UnionKind::All, Box::new(self), other.cast_to())
    }

    pub fn union_distinct<K>(self, other: QueryWithSelect<K>) -> Union<T> {
        Union::And(UnionKind::Distinct, Box::new(self), other.cast_to())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Explain {
    Default,
    Analyze,
}

impl WriteSql for Explain {
    fn write_sql<W: Write>(self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        match self {
            Explain::Default => write!(f, "EXPLAIN "),
            Explain::Analyze => write!(f, "EXPLAIN ANALYZE "),
        }
    }
}
