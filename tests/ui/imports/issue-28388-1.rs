//@ revisions: edition2015 edition2021
//@[edition2015] edition: 2015
//@[edition2021] edition: 2021
// Prefix in imports with empty braces should be resolved and checked privacy, stability, etc.

use foo::{}; //~ ERROR unresolved import `foo`

fn main() {}
