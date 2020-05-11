use crate::binds::BindCount;
use crate::{Column, WriteSql};
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Grouping {
    Col(Column),
    And {
        lhs: Box<Grouping>,
        rhs: Box<Grouping>,
    },
}

impl<T> From<T> for Grouping
where
    T: Into<Column>,
{
    fn from(col: T) -> Self {
        Grouping::Col(col.into())
    }
}

impl WriteSql for Grouping {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Grouping::Col(col) => col.write_sql(f, bind_count),
            Grouping::And { lhs, rhs } => {
                lhs.write_sql(f, bind_count)?;
                write!(f, ", ")?;
                rhs.write_sql(f, bind_count)?;
                Ok(())
            }
        }
    }
}

macro_rules! impl_into_grouping {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Grouping> for ($first, $second)
        where
            $first: Into<Grouping>,
            $second: Into<Grouping>,
        {
            fn into(self) -> Grouping {
                let (lhs, rhs) = self;
                Grouping::And {
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
        impl<$head, $($tail),*> Into<Grouping> for ($head, $($tail),*)
        where
            $head: Into<Grouping>,
            $( $tail: Into<Grouping> ),*
        {
            fn into(self) -> Grouping {
                let (
                    $head, $($tail),*
                ) = self;
                let tail_grouping: Grouping = ($($tail),*).into();

                Grouping::And {
                    lhs: Box::new($head.into()),
                    rhs: Box::new(tail_grouping),
                }
            }
        }

        impl_into_grouping!($($tail),*,);
    };
}

impl_into_grouping!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
