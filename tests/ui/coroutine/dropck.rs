//@ revisions: edition2015 edition2021
//@ [edition2015] edition: 2015..2021
//@ [edition2021] edition: 2021..
#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::cell::RefCell;
use std::ops::Coroutine;
use std::pin::Pin;

fn main() {
    let (mut coro, cell);
    cell = Box::new(RefCell::new(0));
    let ref_ = Box::leak(Box::new(Some(cell.borrow_mut())));
    //~^ ERROR `*cell` does not live long enough [E0597]
    // the upvar is the non-dropck `&mut Option<Ref<'a, i32>>`.
    coro = #[coroutine]
    || {
        // but the coroutine can use it to drop a `Ref<'a, i32>`.
        let _d = ref_.take();
        //[edition2015]~^ ERROR `ref_` does not live long enough [E0597]
        yield;
    };
    Pin::new(&mut coro).resume(());
    // drops the RefCell and then the Ref, leading to use-after-free
}
