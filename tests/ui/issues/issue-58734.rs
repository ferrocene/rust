//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
trait Trait {
    fn exists(self) -> ();

    fn dyn_incompatible() -> Self;
}

impl Trait for () {
    fn exists(self) -> () {
    }

    fn dyn_incompatible() -> Self {
        ()
    }
}

fn main() {
    // dyn-compatible or not, this call is OK
    Trait::exists(());
    // no dyn-compatibility error
    Trait::nonexistent(());
    //[edition2015]~^ WARN trait objects without an explicit `dyn` are deprecated
    //[edition2015]~| WARN this is accepted in the current edition
    //[edition2015]~| ERROR the trait `Trait` is not dyn compatible
    //[edition2021]~^^^^ ERROR: expected a type, found a trait
}
