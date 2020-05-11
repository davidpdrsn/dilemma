use crate::binds::Binds;
use crate::grouping::*;
use crate::ordering::*;
use crate::selection::*;
use crate::*;

pub trait QueryDsl {
    fn select(self, selectable: impl Into<Selection>) -> (String, Binds);

    fn filter(self, filter: impl Into<Filter>) -> Query;

    fn or_filter(self, filter: impl Into<Filter>) -> Query;

    fn join(self, join: PartialJoin) -> Query;

    fn inner_join(self, join: PartialJoin) -> Query;

    fn outer_join(self, join: PartialJoin) -> Query;

    fn group_by(self, grouping: impl Into<Grouping>) -> Query;

    fn then_group_by(self, grouping: impl Into<Grouping>) -> Query;

    fn having(self, having: impl Into<Filter>) -> Query;

    fn and_having(self, having: impl Into<Filter>) -> Query;

    fn or_having(self, having: impl Into<Filter>) -> Query;

    fn order(self, ordering: impl Into<Ordering>) -> Query;

    fn order_by(self, ordering: impl Into<Ordering>) -> Query;

    fn then_order_by(self, ordering: impl Into<Ordering>) -> Query;

    fn limit(self, limit: u64) -> Query;

    fn for_update(self) -> Query;

    fn skip_locked(self) -> Query;

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

    fn or_filter(self, filter: impl Into<Filter>) -> Query {
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

    fn group_by(self, group: impl Into<Grouping>) -> Query {
        let mut query = self.into();
        query.group = Some(group.into());
        query
    }

    fn then_group_by(self, group: impl Into<Grouping>) -> Query {
        let mut query = self.into();
        let new_grouping = match query.group.take() {
            Some(lhs) => Grouping::And {
                lhs: Box::new(lhs),
                rhs: Box::new(group.into()),
            },
            None => group.into(),
        };
        query.group = Some(new_grouping);
        query
    }

    fn having(self, having: impl Into<Filter>) -> Query {
        let mut query = self.into();
        query.having = Some(having.into());
        query
    }

    fn and_having(self, having: impl Into<Filter>) -> Query {
        let mut query = self.into();
        let new_having = if let Some(prev_having) = query.having.take() {
            prev_having.and(having.into())
        } else {
            having.into()
        };
        query.having = Some(new_having);
        query
    }

    fn or_having(self, having: impl Into<Filter>) -> Query {
        let mut query = self.into();
        let new_having = if let Some(prev_having) = query.having.take() {
            prev_having.or(having.into())
        } else {
            having.into()
        };
        query.having = Some(new_having);
        query
    }

    fn order(self, order: impl Into<Ordering>) -> Query {
        self.order_by(order)
    }

    fn order_by(self, order: impl Into<Ordering>) -> Query {
        let mut query = self.into();
        query.order = Some(order.into());
        query
    }

    fn then_order_by(self, order: impl Into<Ordering>) -> Query {
        let mut query = self.into();
        let new_order = match query.order.take() {
            Some(lhs) => Ordering::And {
                lhs: Box::new(lhs),
                rhs: Box::new(order.into()),
            },
            None => order.into(),
        };
        query.order = Some(new_order);
        query
    }

    fn limit(self, limit: u64) -> Query {
        let mut query = self.into();
        query.limit = Some(limit);
        query
    }

    fn for_update(self) -> Query {
        let mut query = self.into();
        query.for_update = true;
        query
    }

    fn skip_locked(self) -> Query {
        let mut query = self.into();
        query.skip_locked = true;
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
        let order = rhs.order.or(lhs.order);
        let group = rhs.group.or(lhs.group);
        let having = rhs.having.or(lhs.having);
        let for_update = lhs.for_update || rhs.for_update;
        let skip_locked = lhs.skip_locked || rhs.skip_locked;

        Query {
            table: lhs.table,
            filter,
            joins: lhs.joins,
            group,
            having,
            order,
            limit,
            for_update,
            skip_locked,
        }
    }
}
