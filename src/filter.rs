use crate::binds::BindCount;
use crate::{expr::Expr, BinOp, WriteSql};
use std::fmt::{self, Write};

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
