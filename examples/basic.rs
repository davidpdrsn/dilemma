#![allow(warnings)]

extern crate compose_sql_2 as compose_sql;

use compose_sql::*;

table! {
    users {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
    }
}

fn main() {
    let query = from(users::table)
        .filter(users::id.eq(123))
        .filter(users::first_name.eq("Bob"))
        .filter(users::last_name.eq("Larsen"))
        .select((users::id, users::first_name, users::last_name));

    println!("{}", query);
}
