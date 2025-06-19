//@ revisions: edition2015 edition2021
//@ [edition2015] edition: 2015..2021
//@ [edition2021] edition: 2021..
//@ [edition2021] check-pass
struct Bar;

impl Bar {
    fn bar(&mut self, _: impl Fn()) {}
}

struct Foo {
    thing: Bar,
    number: usize,
}

impl Foo {
    fn foo(&mut self) {
        self.thing.bar(|| {
        //[edition2015]~^ ERROR cannot borrow `self.thing` as mutable because it is also borrowed as immutable [E0502]
            &self.number;
        });
    }
}

fn main() {}
