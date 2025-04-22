//@ edition: 2018..

pub trait Foo<A=Self> {
    fn foo(&self);
}

pub trait Bar<X=usize, A=Self> {
    fn foo(&self);
}

fn main() {
    let a = <dyn Foo>::lol();
    //~^ ERROR must be explicitly specified
    let b = <dyn Foo<_>>::lol();
    //~^ ERROR no function or associated item named
    let c = <dyn Bar>::lol();
    //~^ ERROR must be explicitly specified
    let d = <dyn Bar<usize, _>>::lol();
    //~^ ERROR no function or associated item named
    let e = <dyn Bar<usize>>::lol();
    //~^ ERROR must be explicitly specified
}
