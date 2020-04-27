#[macro_export]
macro_rules! table {
    (
        $table:ident {
            $(
                $col:ident -> $ty:ident,
            )*
        }
    ) => {
        mod $table {
            use super::*;

            #[derive(Copy, Clone)]
            pub struct table;

            impl From<table> for $crate::Table {
                fn from(t: table) -> Self {
                    Self::new(stringify!($table))
                }
            }

            impl $crate::SelectDsl<table> for $crate::Query {
                fn select(self, t: table) -> String {
                    self.into_sql($crate::Selection::Star($crate::Table::from(t)))
                }
            }

            $(
                #[derive(Copy, Clone)]
                pub struct $col;

                impl SelectDsl<$col> for $crate::Query {
                    fn select(self, t: $col) -> String {
                        self.into_sql($crate::Selection::Column($crate::Column::from(t)))
                    }
                }

                impl From<$col> for $crate::Column {
                    fn from(t: $col) -> Self {
                        Self::new($crate::Table::from(table).name(), stringify!($col))
                    }
                }

                impl From<$col> for $crate::Expr {
                    fn from(t: $col) -> Self {
                        $crate::Expr::Column(t.into())
                    }
                }

                impl $crate::FilterDsl for $col {
                    type SqlType = $ty;
                }
            )*
        }
    };
}
