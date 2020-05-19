use crate::group::*;
use crate::order::*;
use crate::select::*;
use crate::*;

pub trait QueryDsl<T> {
    fn select(self, selectable: impl Into<Select>) -> QueryWithSelect<T>;

    fn filter(self, filter: impl Into<Filter>) -> Query<T>;

    fn or_filter(self, filter: impl Into<Filter>) -> Query<T>;

    fn join<K>(self, join: impl Into<JoinOn<K>>) -> Query<T>;

    fn inner_join<K>(self, join: impl Into<JoinOn<K>>) -> Query<T>;

    fn outer_join<K>(self, join: impl Into<JoinOn<K>>) -> Query<T>;

    fn group_by(self, group: impl Into<Group>) -> Query<T>;

    fn then_group_by(self, group: impl Into<Group>) -> Query<T>;

    fn having(self, having: impl Into<Filter>) -> Query<T>;

    fn and_having(self, having: impl Into<Filter>) -> Query<T>;

    fn or_having(self, having: impl Into<Filter>) -> Query<T>;

    fn order_by(self, order: impl Into<Order>) -> Query<T>;

    fn then_order_by(self, order: impl Into<Order>) -> Query<T>;

    fn limit(self, limit: impl Into<Limit>) -> Query<T>;

    fn offset(self, offset: impl Into<Offset>) -> Query<T>;

    fn distinct(self) -> Query<T>;

    fn distinct_on(self, cols: impl IntoColumns) -> Query<T>;

    fn for_update(self) -> Query<T>;

    fn skip_locked(self) -> Query<T>;

    fn for_key_share(self) -> Query<T>;

    fn for_no_key_update(self) -> Query<T>;

    fn for_share(self) -> Query<T>;

    fn no_wait(self) -> Query<T>;

    fn with(self, ctes: impl Into<Ctes<T>>) -> Query<T>;

    fn explain(self) -> Query<T>;

    fn explain_analyze(self) -> Query<T>;

    fn merge<K>(self, other: impl Into<Query<K>>) -> Query<T>;
}

impl<T, K> QueryDsl<K> for T
where
    T: Into<Query<K>>,
{
    fn select(self, selectable: impl Into<Select>) -> QueryWithSelect<K> {
        QueryWithSelect {
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

    fn inner_join<J>(self, join: impl Into<JoinOn<J>>) -> Query<K> {
        let mut query = self.into();
        query.add_join(join.into().cast_to::<K>(), JoinKind::Inner);
        query
    }

    fn join<J>(self, join: impl Into<JoinOn<J>>) -> Query<K> {
        let mut query = self.into();
        query.add_join(join.into().cast_to::<K>(), JoinKind::Default);
        query
    }

    fn outer_join<J>(self, join: impl Into<JoinOn<J>>) -> Query<K> {
        let mut query = self.into();
        query.add_join(join.into().cast_to::<K>(), JoinKind::Outer);
        query
    }

    fn group_by(self, group: impl Into<Group>) -> Query<K> {
        let mut query = self.into();
        query.group = Some(group.into());
        query
    }

    fn then_group_by(self, group: impl Into<Group>) -> Query<K> {
        let mut query = self.into();
        let new_group = match query.group.take() {
            Some(lhs) => Group::And {
                lhs: Box::new(lhs),
                rhs: Box::new(group.into()),
            },
            None => group.into(),
        };
        query.group = Some(new_group);
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

    fn order_by(self, order: impl Into<Order>) -> Query<K> {
        let mut query = self.into();
        query.order = Some(order.into());
        query
    }

    fn then_order_by(self, order: impl Into<Order>) -> Query<K> {
        let mut query = self.into();
        let mut new_order = order.into();
        match query.order.take() {
            None => {}
            Some(Order::Simple(ordering)) => {
                new_order.add(ordering);
            }
            Some(Order::List(ordering)) => {
                new_order.extend(ordering);
            }
        };
        query.order = Some(new_order);
        query
    }

    fn limit(self, limit: impl Into<Limit>) -> Query<K> {
        let mut query = self.into();
        query.limit = Some(limit.into());
        query
    }

    fn offset(self, offset: impl Into<Offset>) -> Query<K> {
        let mut query = self.into();
        query.offset = Some(offset.into());
        query
    }

    fn distinct(self) -> Query<K> {
        let mut query = self.into();
        query.distinct = Some(Distinct::EachRow);
        query
    }

    fn distinct_on(self, cols: impl IntoColumns) -> Query<K> {
        let mut query = self.into();
        query.distinct = Some(Distinct::On(cols.into_columns()));
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

    fn with(self, ctes: impl Into<Ctes<K>>) -> Query<K> {
        let mut query = self.into();
        query.ctes.extend(ctes.into());
        query
    }

    fn explain(self) -> Query<K> {
        let mut query = self.into();
        query.explain = Some(Explain::Default);
        query
    }

    fn explain_analyze(self) -> Query<K> {
        let mut query = self.into();
        query.explain = Some(Explain::Analyze);
        query
    }

    fn merge<J>(self, other: impl Into<Query<J>>) -> Query<K> {
        let mut lhs = self.into();
        let rhs = other.into();

        lhs.filter = match (lhs.filter, rhs.filter) {
            (Some(a), Some(b)) => Some(Filter::And(Box::new(a), Box::new(b))),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        lhs.joins.extend(rhs.joins.cast_to::<K>());
        lhs.ctes.extend(rhs.ctes.cast_to::<K>());

        lhs
    }
}
