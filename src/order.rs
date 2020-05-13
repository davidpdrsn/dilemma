use crate::binds::BindCount;
use crate::binds::{BindsInternal, CollectBinds};
use crate::{Column, WriteSql};
use itertools::{Itertools, Position};
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Order {
    Simple(Ordering),
    List(Vec<Ordering>),
}

impl Order {
    pub(crate) fn add(&mut self, ordering: Ordering) {
        match self {
            Order::Simple(inner) => {
                *self = Order::List(vec![ordering, inner.clone()]);
            }
            Order::List(inners) => {
                inners.push(ordering);
            }
        }
    }

    pub(crate) fn extend(&mut self, mut ordering: Vec<Ordering>) {
        match self {
            Order::Simple(inner) => {
                ordering.push(inner.clone());
                *self = Order::List(ordering);
            }
            Order::List(inners) => {
                inners.extend(ordering);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ordering {
    Default(Column, Option<NullsPosition>),
    Asc(Column, Option<NullsPosition>),
    Desc(Column, Option<NullsPosition>),
    Raw(String),
}

#[derive(Debug, Clone, Copy)]
pub enum NullsPosition {
    First,
    Last,
}

impl Order {
    pub fn raw(sql: &str) -> Ordering {
        Ordering::Raw(sql.to_string())
    }
}

impl WriteSql for Order {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Order::Simple(inner) => inner.write_sql(f, bind_count),
            Order::List(orderings) => {
                for ordering in orderings.into_iter().with_position() {
                    match ordering {
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

impl WriteSql for Ordering {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        let nulls_position = match self {
            Ordering::Default(col, nulls_position) => {
                col.write_sql(f, bind_count)?;
                nulls_position
            }
            Ordering::Asc(col, nulls_position) => {
                col.write_sql(f, bind_count)?;
                write!(f, " ASC")?;
                nulls_position
            }
            Ordering::Desc(col, nulls_position) => {
                col.write_sql(f, bind_count)?;
                write!(f, " DESC")?;
                nulls_position
            }
            Ordering::Raw(sql) => {
                write!(f, "{}", sql)?;
                &None
            }
        };

        match nulls_position {
            Some(NullsPosition::First) => {
                write!(f, " NULLS FIRST")?;
            }
            Some(NullsPosition::Last) => {
                write!(f, " NULLS LAST")?;
            }
            None => {}
        }

        Ok(())
    }
}

impl From<Ordering> for Order {
    fn from(ordering: Ordering) -> Self {
        Order::Simple(ordering)
    }
}

impl<T> From<T> for Order
where
    T: Into<Column>,
{
    fn from(col: T) -> Self {
        Order::Simple(Ordering::from(col))
    }
}

impl<T> From<T> for Ordering
where
    T: Into<Column>,
{
    fn from(col: T) -> Self {
        Ordering::Default(col.into(), None)
    }
}

pub trait OrderDsl {
    fn asc(self) -> Ordering;

    fn desc(self) -> Ordering;
}

impl<T> OrderDsl for T
where
    T: Into<Column>,
{
    fn asc(self) -> Ordering {
        Ordering::Asc(self.into(), None)
    }

    fn desc(self) -> Ordering {
        Ordering::Desc(self.into(), None)
    }
}

impl OrderDsl for Ordering {
    fn asc(self) -> Ordering {
        match self {
            Ordering::Default(col, nulls)
            | Ordering::Asc(col, nulls)
            | Ordering::Desc(col, nulls) => Ordering::Asc(col, nulls),
            Ordering::Raw(sql) => Ordering::Raw(sql),
        }
    }

    fn desc(self) -> Ordering {
        match self {
            Ordering::Default(col, nulls)
            | Ordering::Asc(col, nulls)
            | Ordering::Desc(col, nulls) => Ordering::Desc(col, nulls),
            Ordering::Raw(sql) => Ordering::Raw(sql),
        }
    }
}

impl CollectBinds for Order {
    fn collect_binds(&self, _: &mut BindsInternal) {}
}

pub trait NullsPositionDsl {
    fn nulls_first(self) -> Ordering;

    fn nulls_last(self) -> Ordering;
}

impl<T> NullsPositionDsl for T
where
    T: Into<Column>,
{
    fn nulls_first(self) -> Ordering {
        Ordering::Default(self.into(), Some(NullsPosition::First))
    }

    fn nulls_last(self) -> Ordering {
        Ordering::Default(self.into(), Some(NullsPosition::Last))
    }
}

impl NullsPositionDsl for Ordering {
    fn nulls_first(self) -> Ordering {
        match self {
            Ordering::Default(inner, _) => Ordering::Default(inner, Some(NullsPosition::First)),
            Ordering::Asc(inner, _) => Ordering::Asc(inner, Some(NullsPosition::First)),
            Ordering::Desc(inner, _) => Ordering::Desc(inner, Some(NullsPosition::First)),
            Ordering::Raw(sql) => Ordering::Raw(sql),
        }
    }

    fn nulls_last(self) -> Ordering {
        match self {
            Ordering::Default(inner, _) => Ordering::Default(inner, Some(NullsPosition::Last)),
            Ordering::Asc(inner, _) => Ordering::Asc(inner, Some(NullsPosition::Last)),
            Ordering::Desc(inner, _) => Ordering::Desc(inner, Some(NullsPosition::Last)),
            Ordering::Raw(sql) => Ordering::Raw(sql),
        }
    }
}

#[allow(warnings)]
impl<T> Into<Order> for (T,)
where
    T: Into<Ordering>,
{
    fn into(self) -> Order {
        Order::Simple(self.0.into())
    }
}

macro_rules! impl_into_order {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Order> for ($first, $second)
        where
            $first: Into<Ordering>,
            $second: Into<Ordering>,
        {
            fn into(self) -> Order {
                let ($first, $second) = self;
                let mut cols = vec![$first.into(), $second.into()];
                Order::List(cols)
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<$head, $($tail),*> Into<Order> for ($head, $($tail),*)
        where
            $head: Into<Ordering>,
            $( $tail: Into<Ordering> ),*
        {
            fn into(self) -> Order {
                let ($head, $($tail),*) = self;
                let mut cols = vec![
                    $head.into(),
                    $( $tail.into(), )*
                ];
                Order::List(cols)
            }
        }

        impl_into_order!($($tail),*,);
    };
}

impl_into_order!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
