use crate::binds::BindCount;
use crate::sql_types::{Integer, Text};
use crate::{Column, Filter, WriteSql};
use std::fmt::{self, Write};

pub trait ExprDsl<SqlType>: Sized {
    fn eq<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>;

    fn ne<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>;

    fn gt<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>;

    fn ge<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>;

    fn lt<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>;

    fn le<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>;
}

impl<T, SqlType> ExprDsl<SqlType> for T
where
    T: IntoExpr<SqlType = SqlType>,
{
    fn eq<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>,
    {
        Filter::BinOp {
            lhs: self.into_expr(),
            op: BinOp::Eq,
            rhs: rhs.into_expr(),
        }
    }

    fn ne<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>,
    {
        Filter::BinOp {
            lhs: self.into_expr(),
            op: BinOp::Ne,
            rhs: rhs.into_expr(),
        }
    }

    fn gt<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>,
    {
        Filter::BinOp {
            lhs: self.into_expr(),
            op: BinOp::Gt,
            rhs: rhs.into_expr(),
        }
    }

    fn ge<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>,
    {
        Filter::BinOp {
            lhs: self.into_expr(),
            op: BinOp::Ge,
            rhs: rhs.into_expr(),
        }
    }

    fn lt<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>,
    {
        Filter::BinOp {
            lhs: self.into_expr(),
            op: BinOp::Lt,
            rhs: rhs.into_expr(),
        }
    }

    fn le<Rhs>(self, rhs: Rhs) -> Filter
    where
        Rhs: IntoExpr<SqlType = SqlType>,
    {
        Filter::BinOp {
            lhs: self.into_expr(),
            op: BinOp::Le,
            rhs: rhs.into_expr(),
        }
    }
}

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

impl IntoExpr for String {
    type SqlType = Text;

    fn into_expr(self) -> Expr {
        Expr::String(self)
    }
}

pub trait IntoExpr {
    type SqlType;

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
    Ne,
    Ge,
    Gt,
    Lt,
    Le,
}

impl WriteSql for BinOp {
    fn write_sql<W: Write>(&self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        match self {
            BinOp::Eq => write!(f, " = "),
            BinOp::Ne => write!(f, " != "),
            BinOp::Ge => write!(f, " >= "),
            BinOp::Gt => write!(f, " > "),
            BinOp::Lt => write!(f, " < "),
            BinOp::Le => write!(f, " <= "),
        }
    }
}
