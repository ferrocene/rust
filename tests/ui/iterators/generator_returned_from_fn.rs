//@ revisions: edition2015 edition2021
//@ [edition2015] edition: 2015..2021
//@ [edition2021] edition: 2021..
#![feature(iter_macro, impl_trait_in_fn_trait_return, yield_expr)]

use std::iter::iter;

fn plain() -> impl Fn() -> impl Iterator<Item = u32> {
    iter! { || {
        yield 0;
        for x in 5..10 {
            yield x * 2;
        }
    } }
}

fn arg() -> impl Fn(u32) -> impl Iterator<Item = u32> {
    iter! { |arg| {
        yield arg;
        for x in 5..10 {
            yield x * 2;
        }
    } }
}

fn capture<'a>(a: &'a u32) -> impl Fn() -> (impl Iterator<Item = u32> + 'a) {
    iter! { || { //[edition2015]~ ERROR cannot return reference to function parameter `a`
        yield *a;
        for x in 5..10 {
            yield x * 2;
        }
    } }
}

fn capture_move(a: &u32) -> impl Fn() -> impl Iterator<Item = u32> {
    iter! { move || {
        //[edition2015]~^ ERROR does not implement `Fn` because it captures
        //[edition2021]~^^ ERROR hidden type for `impl Fn() -> impl Iterator<Item = u32>` captures lifetime that does not appear in bounds
        //[edition2021]~| ERROR hidden type for `impl Iterator<Item = u32>` captures lifetime that does not appear in bounds
        yield *a;
        for x in 5..10 {
            yield x * 2;
        }
    } }
}

fn capture_move_once(a: &u32) -> impl FnOnce() -> impl Iterator<Item = u32> {
    iter! { move || {
        //~^ ERROR captures lifetime
        //~| ERROR: captures lifetime
        yield *a;
        for x in 5..10 {
            yield x * 2;
        }
    } }
}

fn capture_move_once_lifetimes<'a>(
    a: &'a u32,
) -> impl FnOnce() -> (impl Iterator<Item = u32> + 'a) {
    iter! { move || {
        yield *a;
        for x in 5..10 {
            yield x * 2;
        }
    } }
}

fn main() {}
