pub mod a {
    pub trait A {
        fn foo(&self);
    }

}
pub mod b {
    use crate::a::A;

    pub struct B;
    impl A for B { fn foo(&self) {} }

    pub mod c {
        use crate::b::B;

        fn foo(b: &B) {
            b.foo(); //~ ERROR: no method named `foo` found
        }
    }

}

fn main() {}
