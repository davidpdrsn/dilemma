use crate::binds::{BindCount, BindsInternal, CollectBinds};
use crate::from::{CastVecSubQuery, SubQuery};
use crate::write_sql::WriteSql;
use itertools::{Itertools, Position};
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub struct Ctes<T> {
    queries: Vec<SubQuery<T>>,
}

impl<T, K> From<SubQuery<K>> for Ctes<T> {
    fn from(sub_query: SubQuery<K>) -> Self {
        Ctes {
            queries: vec![sub_query.cast_to()],
        }
    }
}

impl<T> Default for Ctes<T> {
    fn default() -> Self {
        Ctes {
            queries: Default::default(),
        }
    }
}

impl<T> Ctes<T> {
    pub fn cast_to<K>(self) -> Ctes<K> {
        Ctes {
            queries: self.queries.cast_to::<K>(),
        }
    }
}

impl<T> WriteSql for &Ctes<T> {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        if self.queries.is_empty() {
            return Ok(());
        }

        write!(f, "WITH ")?;

        for query in self.queries.iter().with_position() {
            match query {
                Position::First(sub_query) | Position::Middle(sub_query) => {
                    write!(f, "\"{}\" AS (", sub_query.alias)?;
                    sub_query.query.write_sql(f, bind_count)?;
                    write!(f, "), ")?;
                }
                Position::Last(sub_query) | Position::Only(sub_query) => {
                    write!(f, "\"{}\" AS (", sub_query.alias)?;
                    sub_query.query.write_sql(f, bind_count)?;
                    write!(f, ") ")?;
                }
            }
        }

        Ok(())
    }
}

impl<T> CollectBinds for Ctes<T> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        for query in &self.queries {
            query.collect_binds(binds)
        }
    }
}

impl<T> IntoIterator for Ctes<T> {
    type Item = <Vec<SubQuery<T>> as IntoIterator>::Item;
    type IntoIter = <Vec<SubQuery<T>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.queries.into_iter()
    }
}

impl<T> Extend<SubQuery<T>> for Ctes<T> {
    fn extend<K>(&mut self, iter: K)
    where
        K: IntoIterator<Item = SubQuery<T>>,
    {
        self.queries.extend(iter.into_iter())
    }
}

#[allow(warnings)]
impl<A, B> Into<Ctes<B>> for (A,)
where
    A: Into<Ctes<B>>,
{
    fn into(self) -> Ctes<B> {
        self.0.into()
    }
}

macro_rules! impl_into_ctes {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<A, $first, $second> Into<Ctes<A>> for ($first, $second)
        where
            $first: Into<Ctes<A>>,
            $second: Into<Ctes<A>>,
        {
            fn into(self) -> Ctes<A> {
                let ($first, $second) = self;
                let mut ctes = Ctes::default();
                ctes.extend($first.into().cast_to::<A>());
                ctes.extend($second.into().cast_to::<A>());
                ctes
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<A, $head, $($tail),*> Into<Ctes<A>> for ($head, $($tail),*)
        where
            $head: Into<Ctes<A>>,
            $($tail: Into<Ctes<A>>),*
        {
            fn into(self) -> Ctes<A> {
                let ($head, $($tail),*) = self;
                let mut ctes = Ctes::default();
                ctes.extend($head.into().cast_to::<A>());
                $(
                    ctes.extend($tail.into().cast_to::<A>());
                )*
                ctes
            }
        }

        impl_into_ctes!($($tail),*,);
    };
}

impl_into_ctes!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
