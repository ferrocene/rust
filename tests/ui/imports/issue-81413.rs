//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

pub const ITEM: Item = Item;

pub struct Item;

pub fn item() {}

pub use doesnt_exist::*;
//~^ ERROR unresolved import `doesnt_exist`
mod a {
    use crate::{item, Item, ITEM};
}

mod b {
    use crate::item;
    use crate::Item;
    use crate::ITEM;
}

mod c {
    use crate::item;
}

fn main() {}
