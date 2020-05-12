use crate::binds::BindCount;
use crate::{Column, Table, WriteSql};
use itertools::Itertools;
use itertools::Position;
use std::fmt::{self, Write};

pub fn star() -> SingleSelection {
    SingleSelection::Star
}

pub fn count(selection: impl Into<SingleSelection>) -> Selection {
    Selection::CountStar(selection.into())
}

#[derive(Debug)]
pub enum Selection {
    CountStar(SingleSelection),
    Simple(SingleSelection),
    List(Vec<SingleSelection>),
    Raw(String),
}

impl Selection {
    pub fn raw(sql: &str) -> Self {
        Selection::Raw(sql.to_string())
    }
}

impl WriteSql for Selection {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Selection::Raw(sql) => write!(f, "{}", sql),
            Selection::Simple(inner) => inner.write_sql(f, bind_count),
            Selection::CountStar(inner) => {
                write!(f, "count(")?;
                inner.write_sql(f, bind_count)?;
                write!(f, ")")?;
                Ok(())
            }
            Selection::List(selections) => {
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
pub enum SingleSelection {
    Star,
    TableStar(Table),
    Column(Column),
    Raw(String),
}

impl SingleSelection {
    pub fn raw(sql: &str) -> Self {
        SingleSelection::Raw(sql.to_string())
    }
}

impl From<SingleSelection> for Selection {
    fn from(selection: SingleSelection) -> Self {
        Selection::Simple(selection)
    }
}

impl WriteSql for SingleSelection {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            SingleSelection::Raw(inner) => write!(f, "{}", inner),
            SingleSelection::Star => write!(f, "*"),
            SingleSelection::TableStar(table) => {
                table.write_sql(f, bind_count)?;
                write!(f, ".*")
            }
            SingleSelection::Column(col) => col.write_sql(f, bind_count),
        }
    }
}

macro_rules! impl_select_dsl {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Selection> for ($first, $second)
        where
            $first: Into<SingleSelection>,
            $second: Into<SingleSelection>,
        {
            fn into(self) -> Selection {
                let ($first, $second) = self;
                let mut cols = vec![$first.into(), $second.into()];
                Selection::List(cols)
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<$head, $($tail),*> Into<Selection> for ($head, $($tail),*)
        where
            $head: Into<SingleSelection>,
            $( $tail: Into<SingleSelection> ),*
        {
            fn into(self) -> Selection {
                let ($head, $($tail),*) = self;
                let mut cols = vec![
                    $head.into(),
                    $( $tail.into(), )*
                ];
                Selection::List(cols)
            }
        }

        impl_select_dsl!($($tail),*,);
    };
}

impl_select_dsl!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
