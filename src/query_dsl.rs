use crate::grouping::*;
use crate::ordering::*;
use crate::selection::*;
use crate::*;

pub trait QueryDsl<T> {
    fn select(self, selectable: impl Into<Selection>) -> QueryWithSelection<T>;

    fn filter(self, filter: impl Into<Filter>) -> Query<T>;

    fn or_filter(self, filter: impl Into<Filter>) -> Query<T>;

    fn join(self, join: impl Into<Join>) -> Query<T>;

    fn inner_join(self, join: impl Into<JoinOn>) -> Query<T>;

    fn outer_join(self, join: impl Into<JoinOn>) -> Query<T>;

    fn group_by(self, grouping: impl Into<Group>) -> Query<T>;

    fn then_group_by(self, grouping: impl Into<Group>) -> Query<T>;

    fn having(self, having: impl Into<Filter>) -> Query<T>;

    fn and_having(self, having: impl Into<Filter>) -> Query<T>;

    fn or_having(self, having: impl Into<Filter>) -> Query<T>;

    fn order(self, ordering: impl Into<Order>) -> Query<T>;

    fn order_by(self, ordering: impl Into<Order>) -> Query<T>;

    fn then_order_by(self, ordering: impl Into<Order>) -> Query<T>;

    fn limit(self, limit: u64) -> Query<T>;

    fn offset(self, offset: u64) -> Query<T>;

    fn for_update(self) -> Query<T>;

    fn skip_locked(self) -> Query<T>;

    fn for_key_share(self) -> Query<T>;

    fn for_no_key_update(self) -> Query<T>;

    fn for_share(self) -> Query<T>;

    fn no_wait(self) -> Query<T>;

    fn merge<K>(self, other: impl Into<Query<K>>) -> Query<T>;
}

impl<T, K> QueryDsl<K> for T
where
    T: Into<Query<K>>,
{
    fn select(self, selectable: impl Into<Selection>) -> QueryWithSelection<K> {
        QueryWithSelection {
            query: self.into(),
            selection: selectable.into(),
        }
    }

    fn filter(self, filter: impl Into<Filter>) -> Query<K> {
        let mut query = self.into();

        query.filter = if let Some(prev_filter) = query.filter.take() {
            Some(Filter::And(Box::new(prev_filter), Box::new(filter.into())))
        } else {
            Some(filter.into())
        };

        query
    }

    fn or_filter(self, filter: impl Into<Filter>) -> Query<K> {
        let mut query = self.into();

        query.filter = if let Some(prev_filter) = query.filter.take() {
            Some(Filter::Or(Box::new(prev_filter), Box::new(filter.into())))
        } else {
            Some(filter.into())
        };

        query
    }

    fn inner_join(self, join: impl Into<JoinOn>) -> Query<K> {
        let mut query = self.into();
        query.add_join(join.into(), JoinKind::Inner);
        query
    }

    fn join(self, join: impl Into<Join>) -> Query<K> {
        let mut query = self.into();
        query.joins.push(join.into());
        query
    }

    fn outer_join(self, join: impl Into<JoinOn>) -> Query<K> {
        let mut query = self.into();
        query.add_join(join.into(), JoinKind::Outer);
        query
    }

    fn group_by(self, group: impl Into<Group>) -> Query<K> {
        let mut query = self.into();
        query.group = Some(group.into());
        query
    }

    fn then_group_by(self, group: impl Into<Group>) -> Query<K> {
        let mut query = self.into();
        let new_grouping = match query.group.take() {
            Some(lhs) => Group::And {
                lhs: Box::new(lhs),
                rhs: Box::new(group.into()),
            },
            None => group.into(),
        };
        query.group = Some(new_grouping);
        query
    }

    fn having(self, having: impl Into<Filter>) -> Query<K> {
        let mut query = self.into();
        query.having = Some(having.into());
        query
    }

    fn and_having(self, having: impl Into<Filter>) -> Query<K> {
        let mut query = self.into();
        let new_having = if let Some(prev_having) = query.having.take() {
            prev_having.and(having.into())
        } else {
            having.into()
        };
        query.having = Some(new_having);
        query
    }

    fn or_having(self, having: impl Into<Filter>) -> Query<K> {
        let mut query = self.into();
        let new_having = if let Some(prev_having) = query.having.take() {
            prev_having.or(having.into())
        } else {
            having.into()
        };
        query.having = Some(new_having);
        query
    }

    fn order(self, order: impl Into<Order>) -> Query<K> {
        self.order_by(order)
    }

    fn order_by(self, order: impl Into<Order>) -> Query<K> {
        let mut query = self.into();
        query.order = Some(order.into());
        query
    }

    fn then_order_by(self, order: impl Into<Order>) -> Query<K> {
        let mut query = self.into();
        let new_order = match query.order.take() {
            Some(lhs) => Order::And {
                lhs: Box::new(lhs),
                rhs: Box::new(order.into()),
            },
            None => order.into(),
        };
        query.order = Some(new_order);
        query
    }

    fn limit(self, limit: u64) -> Query<K> {
        let mut query = self.into();
        query.limit = Some(limit);
        query
    }

    fn offset(self, offset: u64) -> Query<K> {
        let mut query = self.into();
        query.offset = Some(offset);
        query
    }

    fn for_update(self) -> Query<K> {
        let mut query = self.into();
        query.row_locking.for_update = true;
        query
    }

    fn skip_locked(self) -> Query<K> {
        let mut query = self.into();
        query.row_locking.skip_locked = true;
        query
    }

    fn for_key_share(self) -> Query<K> {
        let mut query = self.into();
        query.row_locking.for_key_share = true;
        query
    }

    fn for_no_key_update(self) -> Query<K> {
        let mut query = self.into();
        query.row_locking.for_no_key_update = true;
        query
    }

    fn for_share(self) -> Query<K> {
        let mut query = self.into();
        query.row_locking.for_share = true;
        query
    }

    fn no_wait(self) -> Query<K> {
        let mut query = self.into();
        query.row_locking.no_wait = true;
        query
    }

    fn merge<J>(self, other: impl Into<Query<J>>) -> Query<K> {
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
        let offset = rhs.offset.or(lhs.offset);
        let order = rhs.order.or(lhs.order);
        let group = rhs.group.or(lhs.group);
        let having = rhs.having.or(lhs.having);
        let row_locking = lhs.row_locking.or(rhs.row_locking);

        Query {
            table: lhs.table,
            filter,
            joins: lhs.joins,
            group,
            having,
            order,
            limit,
            offset,
            row_locking,
            _marker: lhs._marker,
        }
    }
}
