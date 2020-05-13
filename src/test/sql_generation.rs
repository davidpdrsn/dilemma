use crate::*;

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
fn select_table_star() {
    let (query, mut binds) = users::table.select(users::star).to_sql();

    assert_eq!(query, r#"SELECT "users".* FROM "users""#);
    assert_eq!(binds.next(), None);
}

#[test]
fn select_star() {
    let (query, mut binds) = users::table.select(star()).to_sql();

    assert_eq!(query, r#"SELECT * FROM "users""#);
    assert_eq!(binds.next(), None);
}

#[test]
fn select_single_column() {
    let (query, mut binds) = users::table.select(users::id).to_sql();

    assert_eq!(query, r#"SELECT "users"."id" FROM "users""#);
    assert_eq!(binds.next(), None);
}

#[test]
fn select_multiple_columns() {
    let (query, mut binds) = users::table
        .select((users::id, users::star, users::country_id))
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users"."id", "users".*, "users"."country_id" FROM "users""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn basic_filter() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1))
        .select(users::star)
        .to_sql();

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
        .select(users::star)
        .to_sql();

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
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1 AND "users"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), None);
}

#[test]
fn query_or_filter() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1))
        .or_filter(users::id.eq(2))
        .select(users::star)
        .to_sql();

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
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1 AND "users"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(2)));
    assert_eq!(binds.next(), None);
}

#[test]
fn or_filter() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1).or(users::id.eq(2)))
        .select(users::star)
        .to_sql();

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
        .select(users::star)
        .to_sql();

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
        .select(users::star)
        .to_sql();

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
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" OUTER JOIN "countries" ON "countries"."id" = "users"."country_id" AND "users"."id" = $1"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), None);
}

#[test]
fn merging_filters() {
    let a = users::table.filter(users::id.eq(1));
    let b = users::table.filter(users::name.eq("Bob"));

    let (query, mut binds) = a.merge(b).select(users::star).to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1 AND "users"."name" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::String("Bob".to_string())));
    assert_eq!(binds.next(), None);
}

#[test]
fn merging_with_joins() {
    let a = users::table.outer_join(countries::table.on(countries::id.eq(users::country_id)));
    let b = users::table.filter(users::name.eq("Bob"));

    let (query, mut binds) = a.merge(b).select(users::star).to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" OUTER JOIN "countries" ON "countries"."id" = "users"."country_id" WHERE "users"."name" = $1"#
    );
    assert_eq!(binds.next(), Some(Bind::String("Bob".to_string())));
    assert_eq!(binds.next(), None);
}

#[test]
fn limit() {
    let (query, mut binds) = users::table.limit(10).select(users::star).to_sql();

    assert_eq!(query, r#"SELECT "users".* FROM "users" LIMIT $1"#);
    assert_eq!(binds.next(), Some(Bind::I32(10)));
    assert_eq!(binds.next(), None);
}

#[test]
fn merge_limit_lhs() {
    let (query, mut binds) = users::table
        .limit(10)
        .merge(users::table.filter(users::id.eq(1)))
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1 LIMIT $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(10)));
    assert_eq!(binds.next(), None);
}

#[test]
fn merge_limit_rhs() {
    let (query, mut binds) = users::table
        .filter(users::id.eq(1))
        .merge(users::table.limit(10))
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" WHERE "users"."id" = $1 LIMIT $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(10)));
    assert_eq!(binds.next(), None);
}

#[test]
fn merge_limit_both() {
    let (query, mut binds) = users::table
        .limit(9999)
        .merge(users::table.limit(10))
        .select(users::star)
        .to_sql();

    assert_eq!(query, r#"SELECT "users".* FROM "users" LIMIT $1"#);
    assert_eq!(binds.next(), Some(Bind::I32(10)));
    assert_eq!(binds.next(), None);
}

#[test]
fn merge_with_table() {
    // These should just type check. SQL generation covered by other tests

    users::table
        .filter(users::id.eq(1))
        .merge(users::table.filter(users::id.eq(1)));

    users::table.filter(users::id.eq(1)).merge(users::table);

    users::table.merge(users::table.filter(users::id.eq(1)));

    users::table.merge(users::table);
}

#[test]
fn constant_expression_i32() {
    let (query, mut binds) = users::table.filter(1.eq(2)).select(users::star).to_sql();

    assert_eq!(query, r#"SELECT "users".* FROM "users" WHERE $1 = $2"#);
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(2)));
    assert_eq!(binds.next(), None);
}

#[test]
fn constant_expression_string() {
    let (query, mut binds) = users::table
        .filter(ExprDsl::eq("foo", "bar"))
        .select(users::star)
        .to_sql();

    assert_eq!(query, r#"SELECT "users".* FROM "users" WHERE $1 = $2"#);
    assert_eq!(binds.next(), Some(Bind::String("foo".to_string())));
    assert_eq!(binds.next(), Some(Bind::String("bar".to_string())));
    assert_eq!(binds.next(), None);
}

#[test]
fn order() {
    let (query, mut binds) = users::table
        .order_by(users::id)
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" ORDER BY "users"."id""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn order_asc() {
    let (query, mut binds) = users::table
        .order_by(users::id.asc())
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" ORDER BY "users"."id" ASC"#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn order_desc() {
    let (query, mut binds) = users::table
        .order_by(users::id.desc())
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" ORDER BY "users"."id" DESC"#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn order_and_limit() {
    let (query, mut binds) = users::table
        .limit(10)
        .order_by(users::id.desc())
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" ORDER BY "users"."id" DESC LIMIT $1"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(10)));
    assert_eq!(binds.next(), None);
}

#[test]
fn multiple_orders() {
    let (query, mut binds) = users::table
        .order_by((users::id, users::name, users::name))
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" ORDER BY "users"."id", "users"."name", "users"."name""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn overriding_order() {
    let (query, mut binds) = users::table
        .order_by((users::id, users::name, users::name))
        .order_by(users::id)
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" ORDER BY "users"."id""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn adding_overriding_order() {
    let (query, mut binds) = users::table
        .order_by(users::id)
        .then_order_by(users::name)
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" ORDER BY "users"."id", "users"."name""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn group_by() {
    let (query, mut binds) = users::table
        .group_by(users::id)
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" GROUP BY "users"."id""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn group_by_multiple() {
    let (query, mut binds) = users::table
        .group_by((users::id, users::name))
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" GROUP BY "users"."id", "users"."name""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn replace_group() {
    let (query, mut binds) = users::table
        .group_by(users::id)
        .group_by(users::name)
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" GROUP BY "users"."name""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn add_group() {
    let (query, mut binds) = users::table
        .group_by(users::id)
        .then_group_by(users::name)
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" GROUP BY "users"."id", "users"."name""#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn having() {
    let (query, mut binds) = users::table
        .having(users::id.eq(1))
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" HAVING "users"."id" = $1"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), None);
}

#[test]
fn replace_having() {
    let (query, mut binds) = users::table
        .having(users::id.eq(1))
        .having(users::id.eq(2))
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" HAVING "users"."id" = $1"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(2)));
    assert_eq!(binds.next(), None);
}

#[test]
fn add_and_having() {
    let (query, mut binds) = users::table
        .having(users::id.eq(1))
        .and_having(users::id.eq(2))
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" HAVING "users"."id" = $1 AND "users"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(2)));
    assert_eq!(binds.next(), None);
}

#[test]
fn add_or_having() {
    let (query, mut binds) = users::table
        .having(users::id.eq(1))
        .or_having(users::id.eq(2))
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" HAVING ("users"."id" = $1) OR "users"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), Some(Bind::I32(2)));
    assert_eq!(binds.next(), None);
}

#[test]
fn for_update() {
    let (query, mut binds) = users::table.for_update().select(users::star).to_sql();

    assert_eq!(query, r#"SELECT "users".* FROM "users" FOR UPDATE"#);
    assert_eq!(binds.next(), None);
}

#[test]
fn skip_locked() {
    let (query, mut binds) = users::table.skip_locked().select(users::star).to_sql();

    assert_eq!(query, r#"SELECT "users".* FROM "users" SKIP LOCKED"#);
    assert_eq!(binds.next(), None);
}

#[test]
fn offset() {
    let (query, mut binds) = users::table.offset(10).select(users::star).to_sql();

    assert_eq!(query, r#"SELECT "users".* FROM "users" OFFSET $1"#);
    assert_eq!(binds.next(), Some(Bind::I32(10)));
    assert_eq!(binds.next(), None);
}

#[test]
fn select_count_star() {
    let (query, mut binds) = users::table.select(count(star())).to_sql();

    assert_eq!(query, r#"SELECT count(*) FROM "users""#);
    assert_eq!(binds.next(), None);
}

#[test]
fn select_count_column() {
    let (query, mut binds) = users::table.select(count(users::id)).to_sql();

    assert_eq!(query, r#"SELECT count("users"."id") FROM "users""#);
    assert_eq!(binds.next(), None);
}

#[test]
fn raw_sql_select() {
    let query = users::table
        .inner_join(Join::raw("INNER JOIN countries on countries.id = 1"))
        .join(Join::raw("left outer join countries on 1=1"))
        .join(Join::raw("join users on 1=2"))
        .filter(Filter::raw("1 = 2 AND 1 not in (1, 2, 3)"))
        .group_by(Group::raw("users.id"))
        .having(Filter::raw("1 = 2"))
        .order_by(Order::raw("id desc"))
        .limit(Limit::raw("10"))
        .offset(Offset::raw("-10"));

    let (sql, mut binds) = query.select(Select::raw("users.*")).to_sql();

    assert_eq!(
        sql,
        r#"SELECT users.* FROM "users" INNER JOIN countries on countries.id = 1 left outer join countries on 1=1 join users on 1=2 WHERE 1 = 2 AND 1 not in (1, 2, 3) GROUP BY users.id HAVING 1 = 2 ORDER BY id desc LIMIT 10 OFFSET -10"#
    );
    assert_eq!(binds.next(), None);
}

#[test]
fn raw_sql_simple_select() {
    let (sql, mut binds) = users::table
        .select((
            Select::raw("countries.*"),
            users::star,
            Select::raw("1 as one"),
        ))
        .to_sql();

    assert_eq!(sql, r#"SELECT countries.*, "users".*, 1 as one FROM "users""#);
    assert_eq!(binds.next(), None);
}
