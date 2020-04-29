#![allow(warnings)]

extern crate compose_sql_2 as compose_sql;

use compose_sql::*;

table! {
    users {
        id -> Integer,
        name -> Text,
        country_id -> Integer,
    }
}

table! {
    countries {
        id -> Integer,
        name -> Text,
    }
}

fn main() {
    let bobs = users::table.filter(users::name.eq("Bob"));
    let denmark = countries::table.filter(countries::name.eq("Denmark"));

    let query = bobs
        .join(countries::table.on(countries::id.eq(users::country_id)))
        .merge(denmark);

    let sql = query.select((users::star, countries::star));
    println!("{}", sql);
}
