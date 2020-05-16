use crate::binds::{BindCount, BindsInternal, CollectBinds};
use crate::from::{CastVecSubQuery, SubQuery};
use crate::write_sql::WriteSql;
use itertools::{Itertools, Position};
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub struct Ctes<T> {
    queries: Vec<SubQuery<T>>,
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

    pub fn push<K>(&mut self, sub_query: SubQuery<K>) {
        self.queries.push(sub_query.cast_to::<T>());
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
