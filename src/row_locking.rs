use crate::binds::BindCount;
use crate::binds::{BindsInternal, CollectBinds};
use crate::write_sql::WriteSql;
use std::fmt::{self, Write};

#[derive(Clone, Copy, Debug)]
pub struct RowLocking {
    pub for_update: bool,
    pub skip_locked: bool,
    pub for_key_share: bool,
    pub for_no_key_update: bool,
    pub for_share: bool,
    pub no_wait: bool,
}

impl RowLocking {
    pub fn new() -> Self {
        Self {
            for_update: false,
            skip_locked: false,
            for_key_share: false,
            for_no_key_update: false,
            for_share: false,
            no_wait: false,
        }
    }

    pub fn or(self, other: RowLocking) -> Self {
        Self {
            for_update: self.for_update || other.for_update,
            skip_locked: self.skip_locked || other.skip_locked,
            for_key_share: self.for_key_share || other.for_key_share,
            for_no_key_update: self.for_no_key_update || other.for_no_key_update,
            for_share: self.for_share || other.for_share,
            no_wait: self.no_wait || other.no_wait,
        }
    }
}

impl CollectBinds for RowLocking {
    fn collect_binds(&self, _: &mut BindsInternal) {}
}

impl WriteSql for &RowLocking {
    fn write_sql<W: Write>(self, f: &mut W, _: &mut BindCount) -> fmt::Result {
        if self.for_update {
            write!(f, " FOR UPDATE")?;
        }

        if self.for_no_key_update {
            write!(f, " FOR NO KEY UPDATE")?;
        }

        if self.for_share {
            write!(f, " FOR SHARE")?;
        }

        if self.for_key_share {
            write!(f, " FOR KEY SHARE")?;
        }

        if self.no_wait {
            write!(f, " NO WAIT")?;
        }

        if self.skip_locked {
            write!(f, " SKIP LOCKED")?;
        }

        Ok(())
    }
}
