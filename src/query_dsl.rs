use crate::binds::Binds;
use crate::{Filter, Join, JoinKind, PartialJoin, Query, Selection};

pub trait QueryDsl {
    fn select<T>(self, selectable: T) -> (String, Binds)
    where
        T: Into<Selection>;

    fn filter(self, filter: impl Into<Filter>) -> Query;

    fn filter_or(self, filter: impl Into<Filter>) -> Query;

    fn inner_join(self, join: PartialJoin) -> Query;

    fn join(self, join: PartialJoin) -> Query;

    fn outer_join(self, join: PartialJoin) -> Query;
}

impl<T> QueryDsl for T
where
    T: Into<Query>,
{
    fn select<K>(self, selectable: K) -> (String, Binds)
    where
        K: Into<Selection>,
    {
        self.into().to_sql(selectable.into())
    }

    fn filter(self, filter: impl Into<Filter>) -> Query {
        let mut query = self.into();

        query.filter = if let Some(prev_filter) = query.filter.take() {
            Some(Filter::And(Box::new(prev_filter), Box::new(filter.into())))
        } else {
            Some(filter.into())
        };

        query
    }

    fn filter_or(self, filter: impl Into<Filter>) -> Query {
        let mut query = self.into();

        query.filter = if let Some(prev_filter) = query.filter.take() {
            Some(Filter::Or(Box::new(prev_filter), Box::new(filter.into())))
        } else {
            Some(filter.into())
        };

        query
    }

    fn inner_join(self, join: PartialJoin) -> Query {
        let mut query = self.into();
        query.joins.push(Join {
            kind: JoinKind::Inner,
            table: join.table,
            filter: join.filter,
        });
        query
    }

    fn join(self, join: PartialJoin) -> Query {
        self.into().inner_join(join)
    }

    fn outer_join(self, join: PartialJoin) -> Query {
        let mut query = self.into();
        query.joins.push(Join {
            kind: JoinKind::Outer,
            table: join.table,
            filter: join.filter,
        });
        query
    }
}
