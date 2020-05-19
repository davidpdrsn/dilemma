use crate::binds::{Bind, Binds};
use crate::QueryWithSelect;

#[cfg(feature = "sqlx-executor")]
pub mod sqlx {
    #![allow(unused_imports)]

    use super::*;
    use ::sqlx::arguments::Arguments;
    use ::sqlx::cursor::Cursor;
    use ::sqlx::executor::RefExecutor;
    use ::sqlx::postgres::{PgQueryAs, PgRow};
    use ::sqlx::prelude::FromRow;
    use ::sqlx::Postgres;
    use ::sqlx::{encode::Encode, Database, Error, Execute, Executor, Type};
    use futures::future::BoxFuture;
    use futures::stream::Stream;
    use std::future::Future;
    use std::marker::PhantomData;
    use std::pin::Pin;

    pub fn query_as<'e, O, T, E>(
        // query: QueryWithSelect<T>,
        sql: &'e str,
        executor: E,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<O>, sqlx::Error>> + 'e>>
    where
        for<'c> O: 'e + FromRow<'c, PgRow<'c>> + Send,
        E: 'e + RefExecutor<'e, Database = Postgres> + Send,
        T: 'e,
    {
        Box::pin(async move {
            // let (sql, binds) = query.to_sql();
            let mut query: ::sqlx::QueryAs<'e, Postgres, O> = ::sqlx::query_as::<Postgres, O>(sql);

            // for bind in binds {
            //     query = match bind {
            //         Bind::String(inner) => query.bind(inner),
            //         Bind::I32(inner) => query.bind(inner),
            //     };
            // }

            query.fetch_all(executor).await
        })
    }

    // pub fn query_as<'q, O, T>(query: QueryWithSelect<T>) -> QueryAs<'q, Postgres, O> {
    //     let (sql, binds) = query.to_sql();

    //     let mut query: QueryAs<Postgres, O> = QueryAs {
    //         query: sql,
    //         query_ref: todo!(),
    //         arguments: <Postgres as Database>::Arguments::default(),
    //         database: PhantomData,
    //         output: PhantomData,
    //     };

    //     for bind in binds {
    //         query = match bind {
    //             Bind::String(inner) => query.bind(inner),
    //             Bind::I32(inner) => query.bind(inner),
    //         };
    //     }

    //     query
    // }

    // #[must_use = "query must be executed to affect database"]
    // pub struct QueryAs<'q, DB, O>
    // where
    //     DB: Database,
    // {
    //     query: String,
    //     query_ref: &'q str,
    //     arguments: <DB as Database>::Arguments,
    //     database: PhantomData<DB>,
    //     output: PhantomData<O>,
    // }

    // impl<'q, DB, O> QueryAs<'q, DB, O>
    // where
    //     DB: Database,
    // {
    //     pub fn bind<T>(mut self, value: T) -> Self
    //     where
    //         T: Type<DB>,
    //         T: Encode<DB>,
    //     {
    //         self.arguments.add(value);
    //         self
    //     }
    // }

    // impl<'q, O> PgQueryAs<'q, O> for QueryAs<'q, Postgres, O> {
    //     fn fetch<'e, E>(self, executor: E) -> futures::stream::BoxStream<'e, ::sqlx::Result<O>>
    //     where
    //         E: 'e + Send + RefExecutor<'e, Database = Postgres>,
    //         O: 'e + Send + Unpin + for<'c> FromRow<'c, PgRow<'c>>,
    //         'q: 'e,
    //     {
    //         todo!()
    //     }

    //     fn fetch_optional<'e, E>(
    //         self,
    //         executor: E,
    //     ) -> futures::future::BoxFuture<'e, ::sqlx::Result<Option<O>>>
    //     where
    //         E: 'e + Send + RefExecutor<'e, Database = Postgres>,
    //         O: 'e + Send + for<'c> FromRow<'c, PgRow<'c>>,
    //         'q: 'e,
    //     {
    //         todo!()
    //     }

    //     fn fetch_one<'e, E>(self, executor: E) -> futures::future::BoxFuture<'e, ::sqlx::Result<O>>
    //     where
    //         E: 'e + Send + RefExecutor<'e, Database = Postgres>,
    //         O: 'e + Send + for<'c> FromRow<'c, PgRow<'c>>,
    //         'q: 'e,
    //     {
    //         todo!()
    //     }

    //     fn fetch_all<'e, E>(
    //         self,
    //         executor: E,
    //     ) -> futures::future::BoxFuture<'e, ::sqlx::Result<Vec<O>>>
    //     where
    //         E: 'e + Send + RefExecutor<'e, Database = Postgres>,
    //         O: 'e + Send + for<'c> FromRow<'c, PgRow<'c>>,
    //         'q: 'e,
    //     {
    //         Box::pin(async move {
    //             let mut cursor = executor.fetch_by_ref(self);
    //             let mut out = Vec::new();

    //             while let Some(row) = cursor.next().await? {
    //                 let obj = O::from_row(&row)?;

    //                 out.push(obj);
    //             }

    //             Ok(out)
    //         })
    //     }
    // }

    // impl<'q, O: Send> Execute<'q, Postgres> for QueryAs<'q, Postgres, O> {
    //     fn into_parts(self) -> (&'q str, Option<<Postgres as Database>::Arguments>) {
    //         todo!("into_parts")
    //     }

    //     fn query_string(&self) -> &'q str {
    //         todo!("query_string")
    //     }
    // }
}
