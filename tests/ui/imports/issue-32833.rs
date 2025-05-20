//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

use bar::Foo; //~ ERROR unresolved import `bar::Foo` [E0432]
              //~^ NOTE no `Foo` in `bar`
mod bar {
    use Foo; //[edition2018]~ ERROR unresolved import `Foo`
             //[edition2018]~^ NOTE no external crate `Foo`
}

fn main() {}
