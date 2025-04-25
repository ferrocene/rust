//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..
//@ [edition2015] check-pass

trait X {
    fn test(x: u32, (
//[edition2015]~^ WARN anonymous parameters are deprecated and will be removed in the next edition
//[edition2015]~^^ WARN this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2018!
    )) {}
//[edition2018]~^ ERROR expected one of `:` or `|`, found `)`
}

fn main() {}
