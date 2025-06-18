//@ revisions: edition2015 edition2018
//@[edition2015] edition: 2015
//@[edition2018] edition: 2018..
mod a {
    pub trait Trait {}
}

mod b {
    use Trait;
    //~^ ERROR unresolved import `Trait`
}

mod c {
    impl Trait for () {} //~ ERROR cannot find trait `Trait` in this scope
}

fn main() {}
