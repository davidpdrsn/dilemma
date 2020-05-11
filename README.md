# dilemma

This is an experimental SQL query builder that favors making SQL queries reusable and composable over guaranteeing type safe queries at compile time. I have found that the advanced type system techniques required to catch invalid queries at compile time often hurts usability and makes it difficult to compose and reuse queries. Using raw SQL everywhere is great for usability but doesn't make it possible to compose queries so you end up repeating yourself a lot.

Its meant as an addition to the ORM/query builder you already use. Running the actual SQL queries is outside the scope of the this project.

It goes without saying that this is very much a proof-of-concept and shouldn't be used for anything serious.

## Example usage

```rust
// Declare our tables. The macro syntax and what is expands to is similar to Diesel.
//
// It defines empty structs for our tables and columns and implements a couple of
// traits to make the DSL methods work.
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

// Just a place to group user related queries
struct UserQueries;

impl UserQueries {
    // Notice this should just returns a `Query`
    // No complex generics getting in the way
    fn named_bob() -> Query {
        users::table.filter(users::name.eq("Bob"))
    }

    fn in_country_named(name: &str) -> Query {
        users::table.inner_join(
            countries::table.on(countries::id
                .eq(users::country_id)
                .and(countries::name.eq(name))),
        )
    }
}

let (query, mut binds) = UserQueries::named_bob()
    // Columns to select are applied last and is what triggers SQL generation
    // The returns the raw SQL and an iterator over the bind params
    .select(users::star);
assert_eq!(
    query,
    r#"SELECT "users".* FROM "users" WHERE "users"."name" = $1"#
);
assert_eq!(
    binds.collect::<Vec<_>>(),
    vec![Bind::String("Bob".to_string())]
);

let (query, mut binds) = UserQueries::in_country_named("Denmark")
    // we can also narrow our selects
    .select((users::id, countries::id));
assert_eq!(
    query,
    r#"SELECT "users"."id", "countries"."id" FROM "users" INNER JOIN "countries" ON "countries"."id" = "users"."country_id" AND "countries"."name" = $1"#
);
assert_eq!(
    binds.collect::<Vec<_>>(),
    vec![Bind::String("Denmark".to_string())]
);

// Having defined the queries separately we're now able to "merge" them together
//
// `Query::merge` makes a new query with all the "joins" and all the "wheres" of both
// queries
let (query, mut binds) = UserQueries::named_bob()
    .merge(UserQueries::in_country_named("Denmark"))
    .select((users::star, countries::star));
assert_eq!(
    query,
    r#"SELECT "users".*, "countries".* FROM "users" INNER JOIN "countries" ON "countries"."id" = "users"."country_id" AND "countries"."name" = $1 WHERE "users"."name" = $2"#
);
assert_eq!(
    binds.collect::<Vec<_>>(),
    vec![
        Bind::String("Denmark".to_string()),
        Bind::String("Bob".to_string()),
    ]
);

// I'm trying to optimize for easy of use, rather than type safety SQL generation. So
// this compiles just fine but wouldn't work at runtime. I recommend you test your queries
// :)
users::table.select(countries::star);
```
