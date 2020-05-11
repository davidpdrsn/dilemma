# TODO

- Tests
  - ~~`Query::merge` tests~~

- Macros
  - ~~Make sure we don't generate warnings about case~~

- More query methods
  - Select query parts
      - ~~LIMIT~~
      - ~~ORDER BY~~
      - ~~GROUP BY~~
      - ~~HAVING~~
      - ~~FOR UPDATE~~
      - ~~SKIP LOCKED~~
      - ~~FOR KEY SHARE~~
      - ~~FOR KEY SHARE~~
      - ~~FOR NO KEY UPDATE~~
      - ~~FOR SHARE~~
      - ~~NOWAIT~~
      - OFFSET
      - EXPLAIN
      - EXPLAIN ANALYZE
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
