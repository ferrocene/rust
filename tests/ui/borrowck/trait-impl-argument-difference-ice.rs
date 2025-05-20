//@ revisions: edition2015 edition2018
//@[edition2015]edition:2015
//@[edition2018]edition:2018..
// Issue https://github.com/rust-lang/rust/issues/123414
trait MemoryUnit {
    extern "C" fn read_word(&mut self) -> u8;
    extern "C" fn read_dword(Self::Assoc<'_>) -> u16;
    //[edition2015]~^ WARNING anonymous parameters are deprecated and will be removed in the next edition
    //[edition2015]~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2018!
    //[edition2015]~| ERROR associated type `Assoc` not found for `Self`
    //[edition2018]~^^^^ ERROR expected one of `!`, `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `<`
}

struct ROM {}

impl MemoryUnit for ROM {
//~^ ERROR not all trait items implemented, missing: `read_word`
    extern "C" fn read_dword(&'_ self) -> u16 {
        //~^ ERROR method `read_dword` has a `&self` declaration in the impl, but not in the trait
        let a16 = self.read_word() as u16;
        //~^ ERROR cannot borrow `*self` as mutable, as it is behind a `&` reference
        let b16 = self.read_word() as u16;
        //~^ ERROR cannot borrow `*self` as mutable, as it is behind a `&` reference

        (b16 << 8) | a16
    }
}

pub fn main() {}
