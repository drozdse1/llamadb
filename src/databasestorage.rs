use databaseinfo::DatabaseInfo;
use std::borrow::Cow;
use std::cmp::Eq;
use std::hash::Hash;

pub trait DatabaseStorage {
    type Info: DatabaseInfo;

    fn scan_table<'a>(&'a self, table: &'a <Self::Info as DatabaseInfo>::Table)
    -> Box<dyn Group<ColumnValue=<Self::Info as DatabaseInfo>::ColumnValue> + 'a>;
}

pub trait Group {
    type ColumnValue: Sized + Clone + Eq + Hash + 'static;

    /// Returns any arbitrary row in the group.
    /// Returns None if the group contains no rows.
    fn get_any_row<'a>(&'a self) -> Option<Cow<'a, [Self::ColumnValue]>>;
    
    fn count(&self) -> u64;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=Cow<'a, [Self::ColumnValue]>> + 'a>;
}
