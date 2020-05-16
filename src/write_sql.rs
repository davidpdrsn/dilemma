use crate::binds::BindCount;
use itertools::{Itertools, Position};
use std::fmt::{self, Write};

pub trait WriteSql {
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result;
}

impl<'a, I, T: 'a> WriteSql for I
where
    I: Iterator<Item = &'a T>,
    &'a T: WriteSql,
{
    fn write_sql<W: Write>(self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        for col in self.with_position() {
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
