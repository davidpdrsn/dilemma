// use crate::*;
// use sqlx::{PgPool, Postgres};

// table! {
//     users {
//         username -> Text,
//     }
// }

// #[derive(Debug, sqlx::FromRow)]
// struct User {
//     username: String,
// }

// #[async_std::test]
// async fn foo() {
//     let pool: PgPool = PgPool::new("postgres://localhost/witter").await.unwrap();
//     let users = users::table.select(star()).fetch_all_as::<User, _>(&pool).await;

//     dbg!(&users);
//     assert_eq!(users.len(), 1);
// }
