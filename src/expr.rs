use crate::binds::BindCount;
use crate::sql_types::{Integer, Text};
use crate::{Column, Filter, WriteSql};
use std::fmt::{self, Write};

pub trait ExprDsl<SqlType>: Sized {
    fn eq<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType>;
}

impl<T, SqlType> ExprDsl<SqlType> for T
where
    T: IntoExpr<SqlType>,
{
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expr {
    Column(Column),
    I32(i32),
    String(String),
}

impl WriteSql for Expr {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Expr::Column(col) => col.write_sql(f, bind_count),
            Expr::I32(_) => bind_count.write_sql(f),
            Expr::String(_) => bind_count.write_sql(f),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinOp {
    Eq,
}

impl WriteSql for BinOp {
    fn write_sql<W: Write>(&self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        match self {
            BinOp::Eq => write!(f, " = "),
        }
    }
}
