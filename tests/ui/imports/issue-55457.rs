//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

use NonExistent; //~ ERROR unresolved import `NonExistent`
use non_existent::non_existent; //~ ERROR unresolved import `non_existent`

#[non_existent]
#[derive(NonExistent)]

struct S;

fn main() {}
