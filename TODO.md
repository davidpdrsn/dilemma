# TODO

- Tests
  - ~~`Query::merge` tests~~

- Macros
  - ~~Make sure we don't generate warnings about case~~

- More query methods
  - ~~Limit~~
  - Group by
  - Having
  - For update
  - Skip locked
  - Explain
  - Explain analyze
  - Sub queries
    - From clause
    - Probably else where, if possible
  - Select expression `SELECT 1 as one`
  - Select `count(*)`
  - Raw SQL nodes
  - `Query::remove_*` methods for each field
  - Add all methods from https://docs.rs/diesel/1.4.4/diesel/query_dsl/trait.QueryDsl.html

- API
  - Make `Filter` a struct with private internals
  - Make `Selection` a struct with private internals
