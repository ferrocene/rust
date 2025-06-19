#![allow(warnings)]

trait A<T> { }

struct B<'a, T:'a>(&'a (dyn A<T>+'a));

trait X { }

impl<'a, T> X for B<'a, T> {}

fn f<'a, T:'static, U>(v: Box<dyn A<T>+'static>) -> Box<dyn X+'static> {
    Box::new(B(&*v)) as Box<dyn X> //~ ERROR cannot return value referencing local data `*v`
}

fn main() {}
