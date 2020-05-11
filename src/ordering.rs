use crate::binds::BindCount;
use crate::{Column, WriteSql};
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Ordering {
    Default(Column),
    Asc(Column),
    Desc(Column),
    And {
        lhs: Box<Ordering>,
        rhs: Box<Ordering>,
    },
}

impl<T> From<T> for Ordering
where
    T: Into<Column>,
{
    fn from(col: T) -> Self {
        Ordering::Default(col.into())
    }
}

impl WriteSql for Ordering {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Ordering::Default(col) => col.write_sql(f, bind_count),
            Ordering::Asc(col) => {
                col.write_sql(f, bind_count)?;
                write!(f, " ASC")
            }
            Ordering::Desc(col) => {
                col.write_sql(f, bind_count)?;
                write!(f, " DESC")
            }
            Ordering::And { lhs, rhs } => {
                lhs.write_sql(f, bind_count)?;
                write!(f, ", ")?;
                rhs.write_sql(f, bind_count)?;
                Ok(())
            }
        }
    }
}

pub trait OrderingDsl {
    fn asc(self) -> Ordering;

    fn desc(self) -> Ordering;
}

impl<T> OrderingDsl for T
where
    T: Into<Column>,
{
    fn asc(self) -> Ordering {
        Ordering::Asc(self.into())
    }

    fn desc(self) -> Ordering {
        Ordering::Desc(self.into())
    }
}

macro_rules! impl_into_ordering {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Ordering> for ($first, $second)
        where
            $first: Into<Ordering>,
            $second: Into<Ordering>,
        {
            fn into(self) -> Ordering {
                let (lhs, rhs) = self;
                Ordering::And {
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
        impl<$head, $($tail),*> Into<Ordering> for ($head, $($tail),*)
        where
            $head: Into<Ordering>,
            $( $tail: Into<Ordering> ),*
        {
            fn into(self) -> Ordering {
                let (
                    $head, $($tail),*
                ) = self;
                let tail_ordering: Ordering = ($($tail),*).into();

                Ordering::And {
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
