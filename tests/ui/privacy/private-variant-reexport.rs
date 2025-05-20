mod m1 {
    pub use crate::E::V; //~ ERROR `V` is only public within the crate, and cannot be re-exported outside
}

mod m2 {
    pub use crate::E::{V}; //~ ERROR `V` is only public within the crate, and cannot be re-exported outside
}

mod m3 {
    pub use crate::E::V::{self}; //~ ERROR `V` is only public within the crate, and cannot be re-exported outside
}

#[deny(unused_imports)]
mod m4 {
    pub use crate::E::*;
    //~^ ERROR glob import doesn't reexport anything
    //~| ERROR unused import: `crate::E::*`
}

enum E { V }

fn main() {}
