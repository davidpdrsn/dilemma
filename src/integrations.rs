#![allow(unused_imports)]

use crate::binds::{Bind, Binds};
use crate::Query;
use crate::QueryWithSelect;
use futures::future::BoxFuture;
use futures::stream::Stream;
use sqlx::arguments::Arguments;
use sqlx::cursor::Cursor;
use sqlx::executor::RefExecutor;
use sqlx::postgres::{PgPool, PgQueryAs, PgRow};
use sqlx::prelude::FromRow;
use sqlx::prelude::Row;
use sqlx::{encode::Encode, Database, Error, Execute, Executor, Type};
use sqlx::{query, query_as, Postgres};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

impl<T> QueryWithSelect<T> {
    pub async fn fetch_all_as<'e, R, E>(self, executor: E) -> Vec<R>
    where
        E: 'e + Send + RefExecutor<'e, Database = Postgres>,
        R: 'e + Send + Unpin + for<'c> FromRow<'c, PgRow<'c>>,
    {
        let (sql, binds): (String, Binds) = self.to_sql();
        fetch_all_raw::<R, _>(&sql, binds, executor).await
    }
}

async fn fetch_all_raw<'q, R, E>(sql: &'q str, binds: Binds, executor: E) -> Vec<R>
where
    E: 'q + Send + RefExecutor<'q, Database = Postgres>,
    R: 'q + Send + Unpin + for<'c> FromRow<'c, PgRow<'c>>,
{
    let mut query = sqlx::query_as::<_, R>(sql);

    for bind in binds {
        match bind {
            Bind::String(s) => {
                query = query.bind(s);
            }
            Bind::I32(i) => {
                query = query.bind(i);
            }
        }
    }

    query.fetch_all(executor).await.unwrap()
}
