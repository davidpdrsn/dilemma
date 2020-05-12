#[macro_export]
macro_rules! table {
    (
        $table:ident {
            $(
                $col:ident -> $ty:ident,
            )*
        }
    ) => {
        #[allow(
            unused_variables,
            dead_code,
            missing_docs,
            non_camel_case_types,
        )]
        mod $table {
            use $crate::sql_types::{Integer, Text};

            pub mod dsl {
                pub use super::table as $table;

                $(
                    pub use super::$col;
                )*
            }

            #[derive(Copy, Clone)]
            pub struct table;

            impl From<table> for $crate::Table {
                fn from(t: table) -> Self {
                    Self::new(stringify!($table))
                }
            }

            impl From<table> for $crate::Query<table> {
                fn from(t: table) -> Self {
                    Self::from($crate::Table::from(t))
                }
            }

            #[derive(Copy, Clone)]
            pub struct star;

            impl Into<$crate::Selection> for star {
                fn into(self) -> $crate::Selection {
                    $crate::Selection::Simple(
                        $crate::SimpleSelection::TableStar($crate::Table::from(table))
                    )
                }
            }

            impl Into<$crate::SimpleSelection> for star {
                fn into(self) -> $crate::SimpleSelection {
                    $crate::SimpleSelection::TableStar($crate::Table::from(table))
                }
            }

            $(
                #[derive(Copy, Clone)]
                pub struct $col;

                impl From<$col> for $crate::Selection {
                    fn from(col: $col) -> $crate::Selection {
                        $crate::Selection::Simple(
                            $crate::SimpleSelection::Column($crate::Column::from(col))
                        )
                    }
                }

                impl From<$col> for $crate::SimpleSelection {
                    fn from(col: $col) -> $crate::SimpleSelection {
                        $crate::SimpleSelection::Column($crate::Column::from(col))
                    }
                }

                impl From<$col> for $crate::Column {
                    fn from(t: $col) -> Self {
                        Self::new($crate::Table::from(table).name(), stringify!($col))
                    }
                }

                impl $crate::IntoExpr for $col {
                    type SqlType = $ty;

                    fn into_expr(self) -> $crate::Expr {
                        $crate::Expr::Column(self.into())
                    }
                }
            )*
        }
    };
}
