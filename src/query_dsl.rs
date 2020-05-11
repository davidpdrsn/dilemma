use crate::binds::Binds;
use crate::{Filter, Join, JoinKind, PartialJoin, Query, Selection};

pub trait QueryDsl {
    fn select(self, selectable: impl Into<Selection>) -> (String, Binds);

    fn filter(self, filter: impl Into<Filter>) -> Query;

    fn filter_or(self, filter: impl Into<Filter>) -> Query;

    fn join(self, join: PartialJoin) -> Query;

    fn inner_join(self, join: PartialJoin) -> Query;

    fn outer_join(self, join: PartialJoin) -> Query;

    fn limit(self, limit: u64) -> Query;

    fn merge(self, other: impl Into<Query>) -> Query;
}

impl<T> QueryDsl for T
where
    T: Into<Query>,
{
    fn select(self, selectable: impl Into<Selection>) -> (String, Binds) {
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

    fn limit(self, limit: u64) -> Query {
        let mut query = self.into();
        query.limit = Some(limit);
        query
    }

    fn merge(self, other: impl Into<Query>) -> Query {
        let mut lhs = self.into();
        let rhs = other.into();

        let filter = match (lhs.filter, rhs.filter) {
            (Some(a), Some(b)) => Some(Filter::And(Box::new(a), Box::new(b))),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        lhs.joins.extend(rhs.joins);

        let limit = rhs.limit.or(lhs.limit);

        Query {
            table: lhs.table,
            filter,
            joins: lhs.joins,
            limit,
        }
    }
}
