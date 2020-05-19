// use sqlx::{PgPool, Postgres};
// use crate::*;

// table! {
//     users {
//         id -> Integer,
//         name -> Text,
//         country_id -> Integer,
//     }
// }

// #[allow(warnings)]
// fn foo() {
//     let pool: PgPool = todo!();

//     async {
//         users::table
//             .filter(users::id.eq(1))
//             .select(users::star)
//             .execute(&pool)
//             .await
//             .unwrap();
//     };
// }
