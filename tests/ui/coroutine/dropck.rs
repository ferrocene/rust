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
        yield;
    };
    Pin::new(&mut coro).resume(());
    // drops the RefCell and then the Ref, leading to use-after-free
}
