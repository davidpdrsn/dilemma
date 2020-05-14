# TODO

- More query methods
  - Select query parts
    - EXPLAIN
    - EXPLAIN ANALYZE
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
  - Common table expressions
  - Renaming tables (maybe). Like `select from users u where u.id = 1`

- API
  - Make `Filter` a struct with private internals
  - Make `Selection` a struct with private internals
