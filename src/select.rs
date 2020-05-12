use crate::binds::BindCount;
use crate::{Column, Table, WriteSql};
use itertools::Itertools;
use itertools::Position;
use std::fmt::{self, Write};

pub fn star() -> Selection {
    Selection::Star
}

pub fn count(selection: impl Into<Selection>) -> Select {
    Select::CountStar(selection.into())
}

#[derive(Debug)]
pub enum Select {
    CountStar(Selection),
    Simple(Selection),
    List(Vec<Selection>),
    Raw(String),
}

impl Select {
    pub fn raw(sql: &str) -> Self {
        Select::Raw(sql.to_string())
    }
}

impl WriteSql for Select {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Select::Raw(sql) => write!(f, "{}", sql),
            Select::Simple(inner) => inner.write_sql(f, bind_count),
            Select::CountStar(inner) => {
                write!(f, "count(")?;
                inner.write_sql(f, bind_count)?;
                write!(f, ")")?;
                Ok(())
            }
            Select::List(selections) => {
                for item in selections.into_iter().with_position() {
                    match item {
                        Position::First(col) | Position::Middle(col) => {
                            col.write_sql(f, bind_count)?;
                            write!(f, ", ")?;
                        }
                        Position::Last(col) | Position::Only(col) => {
                            col.write_sql(f, bind_count)?;
                        }
                    }
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum Selection {
    Star,
    TableStar(Table),
    Column(Column),
    Raw(String),
}

impl Selection {
    pub fn raw(sql: &str) -> Self {
        Selection::Raw(sql.to_string())
    }
}

impl From<Selection> for Select {
    fn from(selection: Selection) -> Self {
        Select::Simple(selection)
    }
}

impl WriteSql for Selection {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Selection::Raw(inner) => write!(f, "{}", inner),
            Selection::Star => write!(f, "*"),
            Selection::TableStar(table) => {
                table.write_sql(f, bind_count)?;
                write!(f, ".*")
            }
            Selection::Column(col) => col.write_sql(f, bind_count),
        }
    }
}

macro_rules! impl_select_dsl {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Select> for ($first, $second)
        where
            $first: Into<Selection>,
            $second: Into<Selection>,
        {
            fn into(self) -> Select {
                let ($first, $second) = self;
                let mut cols = vec![$first.into(), $second.into()];
                Select::List(cols)
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<$head, $($tail),*> Into<Select> for ($head, $($tail),*)
        where
            $head: Into<Selection>,
            $( $tail: Into<Selection> ),*
        {
            fn into(self) -> Select {
                let ($head, $($tail),*) = self;
                let mut cols = vec![
                    $head.into(),
                    $( $tail.into(), )*
                ];
                Select::List(cols)
            }
        }

        impl_select_dsl!($($tail),*,);
    };
}

impl_select_dsl!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
