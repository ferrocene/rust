// MIR doesn't generate an error because the assignment isn't reachable. This
// is OK because the test is here to check that the compiler doesn't ICE (cf.
// #5500).

//@ revisions: edition2015 edition2021
//@ [edition2015] edition: 2015..2021
//@ [edition2021] edition: 2021..
//@ [edition2015] check-pass

struct TrieMapIterator<'a> {
    node: &'a usize
}

fn main() {
    let a = 5;
    let _iter = TrieMapIterator{node: &a};
    _iter.node = &panic!()
    //[edition2021]~^ ERROR mismatched types
}
