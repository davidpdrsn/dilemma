use crate::binds::BindCount;
use crate::binds::{BindsInternal, CollectBinds};
use crate::{expr::Expr, BinOp, UnOp, WriteSql};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Filter {
    BinOp { lhs: Expr, op: BinOp, rhs: Expr },
    UnOp { expr: Expr, op: UnOp },
    And(Box<Filter>, Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
    Raw(String),
}

impl WriteSql for &Filter {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Filter::BinOp { lhs, op, rhs } => {
                lhs.write_sql(f, bind_count)?;
                op.write_sql(f, bind_count)?;
                rhs.write_sql(f, bind_count)?;
            }
            Filter::UnOp { expr, op } => {
                expr.write_sql(f, bind_count)?;
                op.write_sql(f, bind_count)?;
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
            Filter::Raw(sql) => {
                write!(f, "{}", sql)?;
            }
        }

        Ok(())
    }
}

impl Filter {
    pub fn raw(sql: &str) -> Self {
        Filter::Raw(sql.to_string())
    }

    pub fn and(self, rhs: Filter) -> Self {
        Filter::And(Box::new(self), Box::new(rhs))
    }

    pub fn or(self, rhs: Filter) -> Self {
        Filter::Or(Box::new(self), Box::new(rhs))
    }
}

impl CollectBinds for Filter {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            Filter::BinOp { lhs, op: _, rhs } => {
                lhs.collect_binds(binds);
                rhs.collect_binds(binds);
            }
            Filter::UnOp { expr, op: _ } => {
                expr.collect_binds(binds);
            }
            Filter::And(lhs, rhs) => {
                lhs.collect_binds(binds);
                rhs.collect_binds(binds);
            }
            Filter::Or(lhs, rhs) => {
                lhs.collect_binds(binds);
                rhs.collect_binds(binds);
            }
            Filter::Raw(_) => {}
        }
    }
}
