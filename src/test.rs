#[allow(unused_imports)]
use super::*;

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

#[test]
fn select_star() {
    let (query, mut binds) = users::table.select(users::star);

    assert_eq!(query, r#"SELECT "users".* FROM "users""#);
    assert_eq!(binds.next(), None);
}

#[test]
fn select_single_column() {
    let (query, mut binds) = users::table.select(users::id);

    assert_eq!(query, r#"SELECT "users"."id" FROM "users""#);
    assert_eq!(binds.next(), None);
}

#[test]
fn select_multiple_columns() {
    let (query, mut binds) = users::table.select((users::id, users::star, users::country_id));

    assert_eq!(
        query,
        r#"SELECT "users"."id", "users".*, "users"."country_id" FROM "users""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn basic_filter() {
    let (query, mut binds) = users::table.filter(users::id.eq(1)).select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1"#
    );

    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), None);
}

#[test]
fn multiple_filters() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1))
        .filter(users::name.eq("Bob"))
        .select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1 AND "users"."name" = $2"#
    );

    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::String("Bob".to_string())));
    assert_eq!(binds.next(), None);
}

#[test]
fn same_filter_twice() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1))
        .filter(users::id.eq(1))
        .select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1 AND "users"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), None);
}

#[test]
fn query_filter_or() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1))
        .filter_or(users::id.eq(2))
        .select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE ("users"."id" = $1) OR "users"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(2)));
    assert_eq!(binds.next(), None);
}

#[test]
fn filter_and() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1).and(users::id.eq(2)))
        .select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1 AND "users"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(2)));
    assert_eq!(binds.next(), None);
}

#[test]
fn filter_or() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1).or(users::id.eq(2)))
        .select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE ("users"."id" = $1) OR "users"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(2)));
    assert_eq!(binds.next(), None);
}

#[test]
fn inner_join() {
    let (query, mut binds) = users::table
        .inner_join(countries::table.on(countries::id.eq(users::country_id)))
        .select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" INNER JOIN "countries" ON "countries"."id" = "users"."country_id""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn outer_join() {
    let (query, mut binds) = users::table
        .outer_join(countries::table.on(countries::id.eq(users::country_id)))
        .select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" OUTER JOIN "countries" ON "countries"."id" = "users"."country_id""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn complex_join() {
    let (query, mut binds) = users::table
        .outer_join(countries::table.on(countries::id.eq(users::country_id).and(users::id.eq(1))))
        .select(users::star);

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" OUTER JOIN "countries" ON "countries"."id" = "users"."country_id" AND "users"."id" = $1"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
}
