//@ dont-require-annotations: NOTE

use zed::bar;
use zed::baz; //~ ERROR unresolved import `zed::baz` [E0432]
              //~| NOTE no `baz` in `zed`
              //~| HELP a similar name exists in the module
              //~| SUGGESTION bar


mod zed {
    pub fn bar() { println!("bar"); }
    use crate::foo; //~ ERROR unresolved import `crate::foo` [E0432]
             //~^ NOTE no `foo` in the root
}

fn main() {
    crate::zed::foo(); //~ ERROR `foo` is private
    bar();
}
