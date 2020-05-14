# TODO

- More query methods
  - Select query parts
    - EXPLAIN
    - EXPLAIN ANALYZE
    - DISTINCT
    - DISTINCT ON
    - UNION [ ALL | DISTINCT ]
    - INTERSECT [ ALL | DISTINCT ]
    - EXCEPT [ ALL | DISTINCT ]
    - Window functions
  - Expression methods
    - eq any (in)
    - ne all (not in)
    - is not null
    - is null
    - between
    - not between
  - Sub queries
    - In joins
  - Add all methods from https://docs.rs/diesel/1.4.4/diesel/query_dsl/trait.QueryDsl.html

- API
  - Make `Filter` a struct with private internals
  - Make `Selection` a struct with private internals
