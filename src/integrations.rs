use crate::binds::{Bind, Binds};
use crate::QueryWithSelect;
use sqlx::database::HasArguments;
use sqlx::prelude::FromRow;
use sqlx::query_as;
use sqlx::IntoArguments;
use sqlx::{encode::Encode, Database, Executor};

impl<T> QueryWithSelect<T> {
    pub async fn fetch_all_as<'c, O, E, DB>(self, executor: E) -> Vec<O>
    where
        O: for<'r> FromRow<'r, DB::Row> + Send + Unpin,
        E: Executor<'c, Database = DB>,
        DB: Database,
        String: for<'q> Encode<'q, DB>,
        i32: for<'q> Encode<'q, DB>,
        for<'q> <DB as HasArguments<'q>>::Arguments: IntoArguments<'q, DB>,
    {
        let (sql, binds): (String, Binds) = self.to_sql();

        let mut query = query_as::<DB, O>(&sql);

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
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::*;
    use sqlx::PgPool;

    table! {
        users {
            id -> Integer,
            username -> Text,
        }
    }

    #[async_std::test]
    async fn test_something() {
        let pool = PgPool::new("postgres://localhost/witter").await.unwrap();

        #[derive(sqlx::FromRow)]
        struct User {
            id: i32,
            username: String,
        }

        users::table
            .select(users::star)
            .fetch_all_as::<User, _, _>(&pool);
    }
}
