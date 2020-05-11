#![forbid(unknown_lints)]

use crate::binds::{BindCount, BindsInternal, CollectBinds};
use itertools::Itertools;
use itertools::Position;
use std::fmt;
use std::fmt::Write;

mod binds;
mod macros;
mod query_dsl;
mod expr;
mod sql_types;

pub use sql_types::{Integer, Text};
pub use binds::{Bind, Binds};
pub use query_dsl::QueryDsl;
pub use expr::{Expr, BinOp, IntoExpr, ExprDsl};

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
    limit: Option<u64>,
}

impl Query {
    pub fn remove_filters(mut self) -> Self {
        self.filter = None;
        self
    }

    pub fn remove_joins(mut self) -> Self {
        self.joins.clear();
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
            limit: None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Join {
    kind: JoinKind,
    table: Table,
    filter: Filter,
}

impl WriteSql for Join {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        self.kind.write_sql(f, bind_count)?;
        self.table.write_sql(f, bind_count)?;

        write!(f, " ON ")?;

        self.filter.write_sql(f, bind_count)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum JoinKind {
    Inner,
    Outer,
}

impl WriteSql for JoinKind {
    fn write_sql<W: Write>(&self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        match self {
            JoinKind::Inner => write!(f, "INNER JOIN ")?,
            JoinKind::Outer => write!(f, "OUTER JOIN ")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PartialJoin {
    table: Table,
    filter: Filter,
}

pub trait JoinOnDsl {
    fn on(self, filter: Filter) -> PartialJoin;
}

impl<T> JoinOnDsl for T
where
    T: Into<Table>,
{
    fn on(self, filter: Filter) -> PartialJoin {
        PartialJoin {
            table: self.into(),
            filter,
        }
    }
}

#[derive(Debug)]
pub enum Selection {
    Star(Table),
    Column(Column),
    List(Vec<Selection>),
}

impl WriteSql for Selection {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Selection::Star(table) => {
                table.write_sql(f, bind_count)?;
                write!(f, ".*")
            }
            Selection::Column(col) => col.write_sql(f, bind_count),
            Selection::List(cols) => {
                for item in cols.into_iter().with_position() {
                    match item {
                        Position::First(col) | Position::Middle(col) => {
                            col.write_sql(f, bind_count)?;
                            write!(f, ", ")?;
                        }
                        Position::Last(col) | Position::Only(col) => {
                            col.write_sql(f, bind_count)?;
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Filter {
    Op { lhs: Expr, op: BinOp, rhs: Expr },
    And(Box<Filter>, Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
}

impl WriteSql for Filter {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Filter::Op { lhs, op, rhs } => {
                lhs.write_sql(f, bind_count)?;
                op.write_sql(f, bind_count)?;
                rhs.write_sql(f, bind_count)?;
            }
            Filter::And(lhs, rhs) => {
                lhs.write_sql(f, bind_count)?;
                write!(f, " AND ")?;
                rhs.write_sql(f, bind_count)?;
            }
            Filter::Or(lhs, rhs) => {
                write!(f, "(")?;
                lhs.write_sql(f, bind_count)?;
                write!(f, ") OR ")?;
                rhs.write_sql(f, bind_count)?;
            }
        }

        Ok(())
    }
}

impl Filter {
    pub fn and(self, rhs: Filter) -> Self {
        Filter::And(Box::new(self), Box::new(rhs))
    }

    pub fn or(self, rhs: Filter) -> Self {
        Filter::Or(Box::new(self), Box::new(rhs))
    }
}

trait WriteSql {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result;
}

macro_rules! impl_select_dsl {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Selection> for ($first, $second)
        where
            $first: Into<Selection>,
            $second: Into<Selection>,
        {
            fn into(self) -> Selection {
                let ($first, $second) = self;
                let mut cols = vec![$first.into(), $second.into()];
                Selection::List(cols)
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<$head, $($tail),*> Into<Selection> for ($head, $($tail),*)
        where
            $head: Into<Selection>,
            $( $tail: Into<Selection> ),*
        {
            fn into(self) -> Selection {
                let ($head, $($tail),*) = self;
                let mut cols = vec![
                    $head.into(),
                    $( $tail.into(), )*
                ];
                Selection::List(cols)
            }
        }

        impl_select_dsl!($($tail),*,);
    };
}

impl_select_dsl!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);

#[cfg(test)]
mod test;
