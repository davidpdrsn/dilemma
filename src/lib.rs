#![forbid(unknown_lints)]

use itertools::Itertools;
use itertools::Position;
use std::fmt;
use std::fmt::Write;

mod macros;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Table {
    name: &'static str,
}

impl WriteSql for Table {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, " {}", self.name)
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
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, " {}.{}", self.table, self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    table: Table,
    joins: Vec<Join>,
    filter: Option<Filter>,
}

impl Query {
    pub fn remove_filter(mut self) -> Self {
        self.filter = None;
        self
    }

    pub fn remove_joins(mut self) -> Self {
        self.joins.clear();
        self
    }

    pub fn merge(mut self, other: Query) -> Self {
        let filter = match (self.filter, other.filter) {
            (Some(a), Some(b)) => Some(Filter::And(Box::new(a), Box::new(b))),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        self.joins.extend(other.joins);

        Query {
            table: self.table,
            filter,
            joins: self.joins,
        }
    }

    pub fn select<T>(self, selectable: T) -> String
    where
        T: Into<Selection>,
    {
        self.to_sql(selectable.into())
    }

    fn to_sql(&self, selection: Selection) -> String {
        let mut f = String::new();

        let result = (|| -> fmt::Result {
            write!(f, "SELECT")?;
            selection.write_sql(&mut f)?;

            write!(f, " FROM")?;
            self.table.write_sql(&mut f)?;

            for join in &self.joins {
                join.write_sql(&mut f)?;
            }

            if let Some(filter) = &self.filter {
                write!(f, " WHERE")?;
                filter.write_sql(&mut f)?;
            }

            Ok(())
        })();

        result.unwrap();

        f
    }
}

impl From<Table> for Query {
    fn from(table: Table) -> Self {
        Self {
            table: table.into(),
            filter: None,
            joins: Vec::new(),
        }
    }
}

pub trait FilterDsl {
    fn filter(self, filter: impl Into<Filter>) -> Query;

    fn filter_or(self, filter: impl Into<Filter>) -> Query;
}

impl<T> FilterDsl for T
where
    T: Into<Query>,
{
    fn filter(self, filter: impl Into<Filter>) -> Query {
        let mut query = self.into();

        query.filter = if let Some(prev_filter) = query.filter.take() {
            Some(Filter::And(Box::new(prev_filter), Box::new(filter.into())))
        } else {
            Some(filter.into())
        };

        query
    }

    fn filter_or(self, filter: impl Into<Filter>) -> Query {
        let mut query = self.into();

        query.filter = if let Some(prev_filter) = query.filter.take() {
            Some(Filter::Or(Box::new(prev_filter), Box::new(filter.into())))
        } else {
            Some(filter.into())
        };

        query
    }
}

pub trait JoinDsl {
    fn inner_join(self, join: PartialJoin) -> Query;

    fn join(self, join: PartialJoin) -> Query;

    fn outer_join(self, join: PartialJoin) -> Query;
}

impl<T> JoinDsl for T
where
    T: Into<Query>,
{
    fn inner_join(self, join: PartialJoin) -> Query {
        let mut query = self.into();
        query.joins.push(Join {
            kind: JoinKind::Inner,
            table: join.table,
            filter: join.filter,
        });
        query
    }

    fn join(self, join: PartialJoin) -> Query {
        self.into().inner_join(join)
    }

    fn outer_join(self, join: PartialJoin) -> Query {
        let mut query = self.into();
        query.joins.push(Join {
            kind: JoinKind::Outer,
            table: join.table,
            filter: join.filter,
        });
        query
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Join {
    kind: JoinKind,
    table: Table,
    filter: Filter,
}

impl WriteSql for Join {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        self.kind.write_sql(f)?;
        self.table.write_sql(f)?;

        write!(f, " ON")?;

        self.filter.write_sql(f)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum JoinKind {
    Inner,
    Outer,
}

impl WriteSql for JoinKind {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            JoinKind::Inner => write!(f, " INNER")?,
            JoinKind::Outer => write!(f, " OUTER")?,
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
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            Selection::Star(table) => write!(f, " {}.*", table.name),
            Selection::Column(col) => col.write_sql(f),
            Selection::List(cols) => {
                for item in cols.into_iter().with_position() {
                    match item {
                        Position::First(col) | Position::Middle(col) => {
                            col.write_sql(f)?;
                            write!(f, ",")?;
                        }
                        Position::Last(col) | Position::Only(col) => {
                            col.write_sql(f)?;
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
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            Filter::Op { lhs, op, rhs } => {
                lhs.write_sql(f)?;
                op.write_sql(f)?;
                rhs.write_sql(f)?;
            }
            Filter::And(lhs, rhs) => {
                lhs.write_sql(f)?;
                write!(f, " AND")?;
                rhs.write_sql(f)?;
            }
            Filter::Or(lhs, rhs) => {
                write!(f, " (")?;
                lhs.write_sql(f)?;
                write!(f, " ) OR")?;
                rhs.write_sql(f)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expr {
    Column(Column),
    I32(i32),
    String(String),
}

impl WriteSql for Expr {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            Expr::Column(col) => col.write_sql(f),
            Expr::I32(n) => write!(f, " {}", n),
            Expr::String(s) => write!(f, " {:?}", s),
        }
    }
}

pub struct Integer;

pub struct Text;

impl IntoExpr<Integer> for i32 {
    fn into_expr(self) -> Expr {
        Expr::I32(self)
    }
}

impl IntoExpr<Text> for &str {
    fn into_expr(self) -> Expr {
        Expr::String(self.to_string())
    }
}

pub trait IntoExpr<SqlType> {
    fn into_expr(self) -> Expr;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinOp {
    Eq,
}

trait WriteSql {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result;
}

impl WriteSql for BinOp {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            BinOp::Eq => write!(f, " ="),
        }
    }
}

pub trait ExprDsl<SqlType>: IntoExpr<SqlType> + Sized {
    fn eq<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType>,
    {
        Filter::Op {
            lhs: self.into_expr(),
            op: BinOp::Eq,
            rhs: rhs.into_expr(),
        }
    }
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
