//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..

#![feature(allocator_api)]
#![feature(const_trait_impl)]

use core::convert::{From, TryFrom};
//[edition2015]~^ ERROR failed to resolve: you might be missing crate `core`
//[edition2015]~| ERROR failed to resolve: you might be missing crate `core`

use std::pin::Pin;
use std::alloc::Allocator;
impl<T: ?Sized, A: Allocator> const From<Box<T, A>> for Pin<Box<T, A>>
//[edition2018]~^ ERROR const `impl` for trait `From` which is not marked with `#[const_trait]`
//[edition2018]~| ERROR type parameter `T` must be used as the type parameter for some local type
where
    A: 'static,
{}

pub fn main() {}
