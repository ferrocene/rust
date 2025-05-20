//@ edition:2018..

fn main() {
    let _: &dyn Copy + 'static; //~ ERROR ambiguous `+` in a type
    //~^ ERROR is not dyn compatible
    let _: &'static dyn Copy + 'static; //~ ERROR ambiguous `+` in a type
    //~^ ERROR is not dyn compatible
}
