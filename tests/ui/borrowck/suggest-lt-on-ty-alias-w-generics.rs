//@ revisions: preedition2021 postedition2021
//@[preedition2021] edition: ..2018
//@[postedition2021] edition: 2021..

type Lazy<T> = Box<dyn Fn() -> T + 'static>;

fn test(x: &i32) -> Lazy<i32> {
    Box::new(|| {
        //~^ ERROR lifetime may not live long enough
        //[preedition2021]~| ERROR closure may outlive the current function
        *x
    })
}

fn main() {}
