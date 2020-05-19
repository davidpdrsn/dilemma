# TODO

- More query methods
  - Select query parts
    - INTERSECT [ ALL | DISTINCT ]
    - EXCEPT [ ALL | DISTINCT ]
    - Window functions
  - Expression methods
    - eq any (in)
    - ne all (not in)
    - between
    - not between
  - Add all methods from https://docs.rs/diesel/1.4.4/diesel/query_dsl/trait.QueryDsl.html
  - Renaming tables (maybe). Like `select from users u where u.id = 1`
  - Raw SQL nodes with bind params

- API
  - Make all enum types opaque. We don't want to accidentally make users depend on the enum names:
    - `src/join.rs`
    - `src/distinct.rs`
    - `src/order.rs`
    - `src/lib.rs`
    - `src/from.rs`
    - `src/filter.rs`
    - `src/expr.rs`
    - `src/group.rs`
    - `src/select.rs`
    - `src/binds.rs`
