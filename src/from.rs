use crate::binds::{BindCount, BindsInternal, CollectBinds};
use crate::write_sql::WriteSql;
use crate::Query;
use crate::QueryWithSelect;
use crate::Table;
use extend::ext;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum FromClause<T> {
    Table(Table),
    SubQuery(SubQuery<T>),
}

impl<T> FromClause<T> {
    pub fn cast_to<K>(self) -> FromClause<K> {
        match self {
            FromClause::Table(table) => FromClause::Table(table),
            FromClause::SubQuery(sub_query) => FromClause::SubQuery(sub_query.cast_to::<K>()),
        }
    }
}

impl<T> From<Table> for FromClause<T> {
    fn from(table: Table) -> Self {
        FromClause::Table(table)
    }
}

impl<T> From<SubQuery<T>> for FromClause<T> {
    fn from(sub_query: SubQuery<T>) -> Self {
        FromClause::SubQuery(sub_query)
    }
}

impl<T> WriteSql for &FromClause<T> {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            FromClause::Table(table) => table.write_sql(f, bind_count),
            FromClause::SubQuery(sub_query) => sub_query.write_sql(f, bind_count),
        }
    }
}

impl<T> CollectBinds for FromClause<T> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            FromClause::Table(table) => table.collect_binds(binds),
            FromClause::SubQuery(sub_query) => sub_query.collect_binds(binds),
        }
    }
}

pub trait IntoSubQuery<T> {
    fn alias(self, name: &str) -> SubQuery<T>;
}

impl<T> IntoSubQuery<T> for QueryWithSelect<T> {
    fn alias(self, alias: &str) -> SubQuery<T> {
        SubQuery {
            query: Box::new(self.into()),
            alias: alias.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubQuery<T> {
    pub(crate) query: Box<QueryWithSelect<T>>,
    pub(crate) alias: String,
}

impl<T> SubQuery<T> {
    pub fn cast_to<K>(self) -> SubQuery<K> {
        let SubQuery { query, alias } = self;

        SubQuery {
            query: Box::new(query.cast_to::<K>()),
            alias,
        }
    }
}

impl<T> CollectBinds for SubQuery<T> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        self.query.collect_binds(binds)
    }
}

#[ext(pub(crate), name = CastVecSubQuery)]
impl<T> Vec<SubQuery<T>> {
    fn cast_to<K>(self) -> Vec<SubQuery<K>> {
        self.into_iter()
            .map(|sub_query| sub_query.cast_to::<K>())
            .collect()
    }
}

impl<T> WriteSql for &SubQuery<T> {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        write!(f, "(")?;
        self.query.write_sql(f, bind_count)?;
        write!(f, ") \"{}\"", self.alias)?;
        Ok(())
    }
}

pub fn from<T>(sub_query: SubQuery<T>) -> Query<T> {
    Query::from(sub_query)
}
