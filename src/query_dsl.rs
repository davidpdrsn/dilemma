use crate::binds::Binds;
use crate::*;

pub trait QueryDsl {
    fn select(self, selectable: impl Into<Selection>) -> (String, Binds);

    fn filter(self, filter: impl Into<Filter>) -> Query;

    fn or_filter(self, filter: impl Into<Filter>) -> Query;

    fn join(self, join: PartialJoin) -> Query;

    fn inner_join(self, join: PartialJoin) -> Query;

    fn outer_join(self, join: PartialJoin) -> Query;

    fn order(self, ordering: impl Into<Ordering>) -> Query;

    fn order_by(self, ordering: impl Into<Ordering>) -> Query;

    fn then_order_by(self, ordering: impl Into<Ordering>) -> Query;

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

    fn order(self, order: impl Into<Ordering>) -> Query {
        self.order_by(order)
    }

    fn order_by(self, order: impl Into<Ordering>) -> Query {
        let mut query = self.into();
        query.order = Some(order.into());
        query
    }

    fn then_order_by(self, ordering: impl Into<Ordering>) -> Query {
        let mut query = self.into();
        let new_ordering = match query.order.take() {
            Some(lhs) => Ordering::And { lhs: Box::new(lhs), rhs: Box::new(ordering.into()) },
            None => ordering.into(),
        };
        query.order = Some(new_ordering);
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
        let order = rhs.order.or(lhs.order);

        Query {
            table: lhs.table,
            filter,
            joins: lhs.joins,
            order,
            limit,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ordering {
    Default(Column),
    Asc(Column),
    Desc(Column),
    And { lhs: Box<Ordering>, rhs: Box<Ordering> },
}

impl<T> From<T> for Ordering
where
    T: Into<Column>
{
    fn from(col: T) -> Self {
        Ordering::Default(col.into())
    }
}

impl WriteSql for Ordering {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Ordering::Default(col) => {
                col.write_sql(f, bind_count)
            }
            Ordering::Asc(col) => {
                col.write_sql(f, bind_count)?;
                write!(f, " ASC")
            }
            Ordering::Desc(col) => {
                col.write_sql(f, bind_count)?;
                write!(f, " DESC")
            }
            Ordering::And { lhs, rhs } => {
                lhs.write_sql(f, bind_count)?;
                write!(f, ", ")?;
                rhs.write_sql(f, bind_count)?;
                Ok(())
            }
        }
    }
}

pub trait OrderingDsl {
    fn asc(self) -> Ordering;

    fn desc(self) -> Ordering;
}

impl<T> OrderingDsl for T
where
    T: Into<Column>,
{
    fn asc(self) -> Ordering {
        Ordering::Asc(self.into())
    }

    fn desc(self) -> Ordering {
        Ordering::Desc(self.into())
    }
}

macro_rules! impl_into_ordering {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Ordering> for ($first, $second)
        where
            $first: Into<Ordering>,
            $second: Into<Ordering>,
        {
            fn into(self) -> Ordering {
                let (lhs, rhs) = self;
                Ordering::And {
                    lhs: Box::new(lhs.into()),
                    rhs: Box::new(rhs.into()),
                }
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<$head, $($tail),*> Into<Ordering> for ($head, $($tail),*)
        where
            $head: Into<Ordering>,
            $( $tail: Into<Ordering> ),*
        {
            fn into(self) -> Ordering {
                let (
                    $head, $($tail),*
                ) = self;
                let tail_ordering: Ordering = ($($tail),*).into();

                Ordering::And {
                    lhs: Box::new($head.into()),
                    rhs: Box::new(tail_ordering),
                }
            }
        }

        impl_into_ordering!($($tail),*,);
    };
}

impl_into_ordering!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);
