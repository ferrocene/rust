//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

mod options {
    pub struct ParseOptions {}
}

mod parser {
    pub use crate::options::*;
    // Private single import shadows public glob import, but arrives too late for initial
    // resolution of `use parser::ParseOptions` because it depends on that resolution itself.
    #[allow(hidden_glob_reexports)]
    use ParseOptions;
}

pub use parser::ParseOptions; //~ ERROR struct import `ParseOptions` is private

fn main() {}
