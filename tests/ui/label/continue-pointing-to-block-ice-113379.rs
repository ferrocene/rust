//! Regression test for ICE #113379. Liveness linting assumes that `continue`s all point to loops.
//! This tests that if a `continue` points to a block, we don't run liveness lints.

//@ revisions: edition2015 edition2018
//@[edition2015] edition: 2015
//@[edition2018] edition: 2018..

async fn f999() -> Vec<usize> {
    //[edition2015]~^ ERROR `async fn` is not permitted in Rust 2015
    'b: {
        //~^ ERROR mismatched types
        continue 'b;
        //~^ ERROR `continue` pointing to a labeled block
    }
}
//~^ ERROR `main` function not found
