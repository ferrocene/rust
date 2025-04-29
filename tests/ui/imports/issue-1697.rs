//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
// Testing that we don't fail abnormally after hitting the errors

use unresolved::*;
//~^ ERROR unresolved import `unresolved` [E0432]
//~| NOTE use of unresolved module or unlinked crate `unresolved`
//~| HELP you might be missing a crate named `unresolved`

fn main() {}
