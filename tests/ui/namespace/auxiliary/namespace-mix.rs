pub mod c {
    pub struct S {}
    pub struct TS();
    pub struct US;
    pub enum E {
        V {},
        TV(),
        UV,
    }

    pub struct Item;
}

pub mod xm1 {
    pub use crate::c::*;
    pub type S = crate::c::Item;
}
pub mod xm2 {
    pub use crate::c::*;
    pub const S: crate::c::Item = crate::c::Item;
}

pub mod xm3 {
    pub use crate::c::*;
    pub type TS = crate::c::Item;
}
pub mod xm4 {
    pub use crate::c::*;
    pub const TS: crate::c::Item = crate::c::Item;
}

pub mod xm5 {
    pub use crate::c::*;
    pub type US = crate::c::Item;
}
pub mod xm6 {
    pub use crate::c::*;
    pub const US: crate::c::Item = crate::c::Item;
}

pub mod xm7 {
    pub use crate::c::E::*;
    pub type V = crate::c::Item;
}
pub mod xm8 {
    pub use crate::c::E::*;
    pub const V: crate::c::Item = crate::c::Item;
}

pub mod xm9 {
    pub use crate::c::E::*;
    pub type TV = crate::c::Item;
}
pub mod xmA {
    pub use crate::c::E::*;
    pub const TV: crate::c::Item = crate::c::Item;
}

pub mod xmB {
    pub use crate::c::E::*;
    pub type UV = crate::c::Item;
}
pub mod xmC {
    pub use crate::c::E::*;
    pub const UV: crate::c::Item = crate::c::Item;
}
