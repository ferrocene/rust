#![feature(decl_macro)]

mod foo {
    fn f() {}

    pub macro m($e:expr) {
        f();
        self::f();
        crate::foo::f();
        $e
    }
}

fn main() {
    foo::m!(
        foo::f() //~ ERROR `f` is private
    );
}
