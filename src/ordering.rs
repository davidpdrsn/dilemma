use crate::binds::{BindsInternal, CollectBinds};
use crate::binds::BindCount;
use crate::{Column, WriteSql};
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Order {
    Default(Column),
    Asc(Column),
    Desc(Column),
    And {
        lhs: Box<Order>,
        rhs: Box<Order>,
    },
}

impl<T> From<T> for Order
where
    T: Into<Column>,
{
    fn from(col: T) -> Self {
        Order::Default(col.into())
    }
}

impl WriteSql for Order {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Order::Default(col) => col.write_sql(f, bind_count),
            Order::Asc(col) => {
                col.write_sql(f, bind_count)?;
                write!(f, " ASC")
            }
            Order::Desc(col) => {
                col.write_sql(f, bind_count)?;
                write!(f, " DESC")
            }
            Order::And { lhs, rhs } => {
                lhs.write_sql(f, bind_count)?;
                write!(f, ", ")?;
                rhs.write_sql(f, bind_count)?;
                Ok(())
            }
        }
    }
}

impl CollectBinds for Order {
    fn collect_binds(&self, _: &mut BindsInternal) {}
}

pub trait OrderDsl {
    fn asc(self) -> Order;

    fn desc(self) -> Order;
}

impl<T> OrderDsl for T
where
    T: Into<Column>,
{
    fn asc(self) -> Order {
        Order::Asc(self.into())
    }

    fn desc(self) -> Order {
        Order::Desc(self.into())
    }
}

macro_rules! impl_into_ordering {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Order> for ($first, $second)
        where
            $first: Into<Order>,
            $second: Into<Order>,
        {
            fn into(self) -> Order {
                let (lhs, rhs) = self;
                Order::And {
                    lhs: Box::new(lhs.into()),
                    rhs: Box::new(rhs.into()),
                }
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<$head, $($tail),*> Into<Order> for ($head, $($tail),*)
        where
            $head: Into<Order>,
            $( $tail: Into<Order> ),*
        {
            fn into(self) -> Order {
                let (
                    $head, $($tail),*
                ) = self;
                let tail_ordering: Order = ($($tail),*).into();

                Order::And {
                    lhs: Box::new($head.into()),
                    rhs: Box::new(tail_ordering),
                }
            }
        }

        impl_into_ordering!($($tail),*,);
    };
}

impl_into_ordering!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
