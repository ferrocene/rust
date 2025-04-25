//@ edition: ..2024

#![deny(rust_2024_compatibility)]

fn gen() {}
//~^ ERROR `gen` is a keyword in the 2024 edition
//~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2024!

fn main() {
    let gen = r#gen;
    //~^ ERROR `gen` is a keyword in the 2024 edition
    //~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2024!
}

macro_rules! t {
    () => { mod test { fn gen() {} } }
    //~^ ERROR `gen` is a keyword in the 2024 edition
    //~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2024!
}

fn test<'gen>(_: &'gen i32) {}
//~^ ERROR `gen` is a keyword in the 2024 edition
//~| ERROR `gen` is a keyword in the 2024 edition
//~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2024!
//~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2024!

struct Test<'gen>(Box<Test<'gen>>, &'gen ());
//~^ ERROR `gen` is a keyword in the 2024 edition
//~| ERROR `gen` is a keyword in the 2024 edition
//~| ERROR `gen` is a keyword in the 2024 edition
//~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2024!
//~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2024!
//~| WARNING this is accepted in the current edition (Rust YYYY) but is a hard error in Rust 2024!

t!();
