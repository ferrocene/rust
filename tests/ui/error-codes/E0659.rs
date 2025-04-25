mod moon {
    pub fn foo() {}
}

mod earth {
    pub fn foo() {}
}

mod collider {
    pub use crate::earth::*;
    pub use crate::moon::*;
}

fn main() {
    collider::foo(); //~ ERROR E0659
}
