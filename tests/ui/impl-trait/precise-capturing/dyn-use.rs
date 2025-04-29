//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
//
fn dyn() -> &'static dyn use<> { &() }
//[edition2015]~^ ERROR expected one of `!`, `(`, `::`, `<`, `where`, or `{`, found keyword `use`
//[edition2021]~^^ ERROR expected identifier
//[edition2021]~| ERROR precise capturing syntax not allowed
//[edition2021]~| ERROR at least one trait is required

fn main() {}
