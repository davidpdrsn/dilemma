use crate::binds::BindCount;
use crate::Column;
use itertools::{Itertools, Position};
use std::fmt::{self, Write};

pub trait WriteSql {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result;
}

impl WriteSql for Vec<Column> {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        for col in self.into_iter().with_position() {
            match col {
                Position::First(col) | Position::Middle(col) => {
                    col.write_sql(f, bind_count)?;
                    write!(f, ", ")?;
                }
                Position::Last(col) | Position::Only(col) => {
                    col.write_sql(f, bind_count)?;
                }
            }
        }

        Ok(())
    }
}
