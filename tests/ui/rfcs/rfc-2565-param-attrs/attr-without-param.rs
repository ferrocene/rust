//@ revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2018..

#[cfg(false)]
impl S {
    fn f(#[attr]) {} //~ ERROR expected parameter name, found `)`
}

#[cfg(false)]
impl T for S {
    fn f(#[attr]) {}
    //~^ ERROR expected parameter name, found `)`
}

#[cfg(false)]
trait T {
    fn f(#[attr]);
    //[edition2015]~^ ERROR expected argument name, found `)`
    //[postedition2015]~^^ ERROR expected parameter name, found `)`
}

fn main() {}
