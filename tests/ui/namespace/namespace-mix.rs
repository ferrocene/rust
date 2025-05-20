//@ aux-build:namespace-mix.rs
//@ revisions: edition2015 postedition2015
//@[edition2015] edition: 2015
//@[postedition2015] edition: 2018..

extern crate namespace_mix;
use namespace_mix::*;

mod c {
    pub struct S {}
    pub struct TS();
    pub struct US;
    pub enum E {
        V {},
        TV(),
        UV,
    }

    pub struct Item;
}

// Use something emitting the type argument name, e.g., unsatisfied bound.
trait Impossible {}
fn check<T: Impossible>(_: T) {}

mod m1 {
    pub use crate::c::*;
    pub type S = crate::c::Item;
}
mod m2 {
    pub use crate::c::*;
    pub const S: crate::c::Item = crate::c::Item;
}

fn f12() {
    check(m1::S{}); //~ ERROR c::Item
    check(m1::S); //~ ERROR expected value, found type alias `m1::S`
    check(m2::S{}); //~ ERROR c::S
    check(m2::S); //~ ERROR c::Item
}
fn xf12() {
    check(xm1::S{}); //~ ERROR c::Item
    check(xm1::S); //~ ERROR expected value, found type alias `xm1::S`
    check(xm2::S{}); //~ ERROR c::S
    check(xm2::S); //~ ERROR c::Item
}

mod m3 {
    pub use crate::c::*;
    pub type TS = crate::c::Item;
}
mod m4 {
    pub use crate::c::*;
    pub const TS: crate::c::Item = crate::c::Item;
}

fn f34() {
    check(m3::TS{}); //~ ERROR c::Item
    check(m3::TS); //~ ERROR c::TS
    check(m4::TS{}); //~ ERROR c::TS
    check(m4::TS); //~ ERROR c::Item
}
fn xf34() {
    check(xm3::TS{}); //~ ERROR c::Item
    check(xm3::TS); //~ ERROR c::TS
    check(xm4::TS{}); //~ ERROR c::TS
    check(xm4::TS); //~ ERROR c::Item
}

mod m5 {
    pub use crate::c::*;
    pub type US = crate::c::Item;
}
mod m6 {
    pub use crate::c::*;
    pub const US: crate::c::Item = crate::c::Item;
}

fn f56() {
    check(m5::US{}); //~ ERROR c::Item
    check(m5::US); //~ ERROR c::US
    check(m6::US{}); //~ ERROR c::US
    check(m6::US); //~ ERROR c::Item
}
fn xf56() {
    check(xm5::US{}); //~ ERROR c::Item
    check(xm5::US); //~ ERROR c::US
    check(xm6::US{}); //~ ERROR c::US
    check(xm6::US); //~ ERROR c::Item
}

mod m7 {
    pub use crate::c::E::*;
    pub type V =crate ::c::Item;
}
mod m8 {
    pub use crate::c::E::*;
    pub const V: crate::c::Item = crate::c::Item;
}

fn f78() {
    check(m7::V{}); //~ ERROR c::Item
    check(m7::V); //~ ERROR expected value, found type alias `m7::V`
    check(m8::V{}); //~ ERROR c::E
    check(m8::V); //~ ERROR c::Item
}
fn xf78() {
    check(xm7::V{}); //~ ERROR c::Item
    check(xm7::V); //~ ERROR expected value, found type alias `xm7::V`
    check(xm8::V{}); //~ ERROR c::E
    check(xm8::V); //~ ERROR c::Item
}

mod m9 {
    pub use crate::c::E::*;
    pub type TV = crate::c::Item;
}
mod mA {
    pub use crate::c::E::*;
    pub const TV: crate::c::Item = crate::c::Item;
}

fn f9A() {
    check(m9::TV{}); //~ ERROR c::Item
    check(m9::TV); //~ ERROR c::E
    check(mA::TV{}); //~ ERROR c::E
    check(mA::TV); //~ ERROR c::Item
}
fn xf9A() {
    check(xm9::TV{}); //~ ERROR c::Item
    check(xm9::TV); //~ ERROR c::E
    check(xmA::TV{}); //~ ERROR c::E
    check(xmA::TV); //~ ERROR c::Item
}

mod mB {
    pub use crate::c::E::*;
    pub type UV = crate::c::Item;
}
mod mC {
    pub use crate::c::E::*;
    pub const UV: crate::c::Item = crate::c::Item;
}

fn fBC() {
    check(mB::UV{}); //~ ERROR c::Item
    check(mB::UV); //~ ERROR c::E
    check(mC::UV{}); //~ ERROR c::E
    check(mC::UV); //~ ERROR c::Item
}
fn xfBC() {
    check(xmB::UV{}); //~ ERROR c::Item
    check(xmB::UV); //~ ERROR c::E
    check(xmC::UV{}); //~ ERROR c::E
    check(xmC::UV); //~ ERROR c::Item
}

fn main() {}
