use crate::binds::BindCount;
use crate::binds::{BindsInternal, CollectBinds};
use crate::from::FromClause;
use crate::{filter::Filter, WriteSql};
use extend::ext;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub enum Join<T> {
    Known {
        kind: JoinKind,
        from: FromClause<T>,
        filter: Filter,
    },
    RawWithKind(String),
    Raw(String),
}

impl<T> Join<T> {
    pub fn raw(sql: &str) -> JoinOn<T> {
        JoinOn::Raw(sql.to_string())
    }

    pub fn cast_to<K>(self) -> Join<K> {
        match self {
            Join::Known { kind, from, filter } => Join::Known {
                kind,
                from: from.cast_to::<K>(),
                filter,
            },
            Join::RawWithKind(sql) => Join::RawWithKind(sql),
            Join::Raw(sql) => Join::Raw(sql),
        }
    }
}

impl<T> WriteSql for &Join<T> {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Join::Known { kind, from, filter } => {
                kind.write_sql(f, bind_count)?;
                from.write_sql(f, bind_count)?;
                write!(f, " ON ")?;
                filter.write_sql(f, bind_count)?;
            }
            Join::RawWithKind(sql) => {
                write!(f, "{}", sql)?;
            }
            Join::Raw(sql) => {
                write!(f, "{}", sql)?;
            }
        }

        Ok(())
    }
}

impl<T> CollectBinds for Join<T> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        match self {
            Join::Known {
                kind: _,
                from,
                filter,
            } => {
                from.collect_binds(binds);
                filter.collect_binds(binds);
            }
            Join::RawWithKind(_) => {}
            Join::Raw(_) => {}
        }
    }
}

impl<T> CollectBinds for Vec<Join<T>> {
    fn collect_binds(&self, binds: &mut BindsInternal) {
        for join in self {
            join.collect_binds(binds)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum JoinKind {
    Default,
    Inner,
    Outer,
}

impl WriteSql for &JoinKind {
    fn write_sql<W: Write>(self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        match self {
            JoinKind::Default => write!(f, "JOIN ")?,
            JoinKind::Inner => write!(f, "INNER JOIN ")?,
            JoinKind::Outer => write!(f, "OUTER JOIN ")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum JoinOn<T> {
    Known { from: FromClause<T>, filter: Filter },
    Raw(String),
}

impl<T> JoinOn<T> {
    pub fn cast_to<K>(self) -> JoinOn<K> {
        match self {
            JoinOn::Known { from, filter } => JoinOn::Known {
                from: from.cast_to::<K>(),
                filter,
            },
            JoinOn::Raw(sql) => JoinOn::Raw(sql),
        }
    }
}

pub trait JoinOnDsl<T> {
    fn on(self, filter: Filter) -> JoinOn<T>;
}

impl<T, K> JoinOnDsl<K> for T
where
    T: Into<FromClause<K>>,
{
    fn on(self, filter: Filter) -> JoinOn<K> {
        JoinOn::Known {
            from: FromClause::from(self.into()),
            filter,
        }
    }
}

#[ext(pub(crate), name = CastVecJoin)]
impl<T> Vec<Join<T>> {
    fn cast_to<K>(self) -> Vec<Join<K>> {
        self.into_iter().map(|join| join.cast_to::<K>()).collect()
    }
}
