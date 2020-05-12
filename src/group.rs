use crate::binds::BindCount;
use crate::binds::{BindsInternal, CollectBinds};
use crate::{Column, WriteSql};
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Group {
    Col(Column),
    And {
        lhs: Box<Group>,
        rhs: Box<Group>,
    },
    Raw(String),
}

impl Group {
    pub fn raw(sql: &str) -> Self {
        Group::Raw(sql.to_string())
    }
}

impl<T> From<T> for Group
where
    T: Into<Column>,
{
    fn from(col: T) -> Self {
        Group::Col(col.into())
    }
}

impl WriteSql for Group {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Group::Col(col) => col.write_sql(f, bind_count),
            Group::And { lhs, rhs } => {
                lhs.write_sql(f, bind_count)?;
                write!(f, ", ")?;
                rhs.write_sql(f, bind_count)?;
                Ok(())
            }
            Group::Raw(sql) => write!(f, "{}", sql)
        }
    }
}

impl CollectBinds for Group {
    fn collect_binds(&self, _: &mut BindsInternal) {}
}

macro_rules! impl_into_group {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Group> for ($first, $second)
        where
            $first: Into<Group>,
            $second: Into<Group>,
        {
            fn into(self) -> Group {
                let (lhs, rhs) = self;
                Group::And {
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
        impl<$head, $($tail),*> Into<Group> for ($head, $($tail),*)
        where
            $head: Into<Group>,
            $( $tail: Into<Group> ),*
        {
            fn into(self) -> Group {
                let (
                    $head, $($tail),*
                ) = self;
                let tail_group: Group = ($($tail),*).into();

                Group::And {
                    lhs: Box::new($head.into()),
                    rhs: Box::new(tail_group),
                }
            }
        }

        impl_into_group!($($tail),*,);
    };
}

impl_into_group!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
