//@ check-pass

#![allow(long_running_const_eval)]

//@ compile-flags: -Z tiny-const-eval-limit -Z deduplicate-diagnostics=yes
const FOO: () = {
    let mut i = 0;
    loop {
        //~^ WARN is taking a long time
        //~| WARN is taking a long time
        //~| WARN is taking a long time
        //~| WARN is taking a long time
        //~| WARN is taking a long time
        if i == 1000 {
            break;
        } else {
            i += 1;
        }
    }
};

fn main() {
    FOO
}
