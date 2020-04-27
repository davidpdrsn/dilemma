use itertools::Itertools;
use itertools::Position;
use std::fmt;
use std::fmt::Write;

mod macros;

#[derive(Debug, Clone)]
pub struct Table {
    name: &'static str,
}

impl Table {
    pub fn new(name: &'static str) -> Self {
        Table { name }
    }

    pub fn name(&self) -> &'static str {
        &self.name
    }
}

#[derive(Debug, Clone)]
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
        write!(f, "{}.{}", self.table, self.name)
    }
}

pub fn from(table: impl Into<Table>) -> Query {
    Query {
        table: table.into(),
        filters: Vec::new(),
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    table: Table,
    filters: Vec<Filter>,
}

impl Query {
    pub fn filter(mut self, filter: impl Into<Filter>) -> Self {
        self.filters.push(filter.into());
        self
    }

    pub fn into_sql(self, selection: Selection) -> String {
        let mut f = String::new();

        let result = (|| -> fmt::Result {
            write!(f, "select ")?;
            selection.write_sql(&mut f)?;

            write!(f, " from {}", self.table.name)?;
            self.filters.write_sql(&mut f)?;

            Ok(())
        })();

        result.unwrap();

        f
    }
}

#[derive(Debug)]
pub enum Selection {
    Star(Table),
    Column(Column),
    Columns(Vec<Column>),
}

impl WriteSql for Selection {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            Selection::Star(table) => write!(f, "{}.*", table.name),
            Selection::Column(col) => col.write_sql(f),
            Selection::Columns(cols) => {
                for item in cols.into_iter().with_position() {
                    match item {
                        Position::First(col) | Position::Middle(col) => {
                            write!(f, "{}.{}, ", col.table, col.name)?;
                        }
                        Position::Last(col) | Position::Only(col) => {
                            write!(f, "{}.{}", col.table, col.name)?;
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Filter {
    lhs: Expr,
    op: BinOp,
    rhs: Expr,
}

impl WriteSql for Vec<Filter> {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        if !self.is_empty() {
            write!(f, " where ")?;

            for item in self.into_iter().with_position() {
                match item {
                    Position::First(filter) | Position::Middle(filter) => {
                        filter.lhs.write_sql(f)?;
                        filter.op.write_sql(f)?;
                        filter.rhs.write_sql(f)?;
                        write!(f, " and ")?;
                    }
                    Position::Last(filter) | Position::Only(filter) => {
                        filter.lhs.write_sql(f)?;
                        filter.op.write_sql(f)?;
                        filter.rhs.write_sql(f)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Column(Column),
    I32(i32),
    String(String),
}

impl WriteSql for Expr {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            Expr::Column(col) => col.write_sql(f),
            Expr::I32(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "{:?}", s),
        }
    }
}

pub struct Integer;

pub struct Text;

impl IntoExpr for i32 {
    type SqlType = Integer;

    fn into_expr(self) -> Expr {
        Expr::I32(self)
    }
}

impl IntoExpr for &str {
    type SqlType = Text;

    fn into_expr(self) -> Expr {
        Expr::String(self.to_string())
    }
}

pub trait IntoExpr {
    type SqlType;

    fn into_expr(self) -> Expr;
}

#[derive(Debug, Copy, Clone)]
pub enum BinOp {
    Eq,
}

trait WriteSql {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result;
}

impl WriteSql for BinOp {
    fn write_sql<W: Write>(&self, f: &mut W) -> fmt::Result {
        match self {
            BinOp::Eq => write!(f, " = "),
        }
    }
}

pub trait FilterDsl: Into<Expr> {
    type SqlType;

    fn eq<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = Self::SqlType>,
    {
        Filter {
            lhs: self.into(),
            op: BinOp::Eq,
            rhs: rhs.into_expr(),
        }
    }
}

pub trait SelectDsl<T>: Sized {
    fn select(self, selection: T) -> String;
}

macro_rules! impl_select_dsl {
    (
        $first:ident, $second:ident,
    ) => {
        impl<$first, $second> SelectDsl<($first, $second)> for Query
        where
            Column: From<$first>,
            Column: From<$second>,
        {
            fn select(self, ($first, $second): ($first, $second)) -> String {
                self.into_sql(Selection::Columns(vec![Column::from($first), Column::from($second)]))
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        impl<$head, $($tail),*> SelectDsl<($head, $($tail),*)> for Query
        where
            Column: From<$head>,
            $( Column: From<$tail> ),*
        {
            fn select(self, ($head, $($tail),*): ($head, $($tail),*)) -> String {
                self.into_sql(Selection::Columns(vec![
                    Column::from($head),
                    $( Column::from($tail) ),*
                ]))
            }
        }

        impl_select_dsl!($($tail),*,);
    };
}

impl_select_dsl!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
