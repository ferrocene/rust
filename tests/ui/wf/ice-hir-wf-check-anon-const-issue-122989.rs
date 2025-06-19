// Regression test for ICE #122989
trait Foo<const N: dyn Bar<2>> {
    //~^ ERROR cycle detected when computing type of `Foo::N`
    //~| ERROR cycle detected when computing type of `Foo::N`
    fn func() {}
}

trait Bar<const M: dyn Foo<2>> {}

fn main() {}
