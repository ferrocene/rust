//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..
use Foo; //~ ERROR unresolved

use Foo1; //~ ERROR unresolved

fn main() {}
