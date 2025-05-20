//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

fn main() {
    ::foo()
    //[edition2015]~^ ERROR cannot find external crate `foo` in the crate root
    //[edition2018]~^^ ERROR cannot find crate `foo` in the list of imported crates
}
