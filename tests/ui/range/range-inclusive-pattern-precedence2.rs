//@ revisions: edition2015 edition2021
//@ [edition2015] edition: 2015..2021
//@ [edition2021] edition: 2021..
// We are going to disallow `&a..=b` and `box a..=b` in a pattern. However, the
// older ... syntax is still allowed as a stability guarantee.

#![feature(box_patterns)]
#![warn(ellipsis_inclusive_range_patterns)]

fn main() {
    match Box::new(12) {
        // FIXME: can we add suggestions like `&(0..=9)`?
        #[cfg(edition2015)]
        box 0...9 => {}
        //[edition2015]~^ WARN `...` range patterns are deprecated
        //[edition2015]~| WARN this is accepted in the current edition
        //[edition2015]~| HELP use `..=` for an inclusive range
        box 10..=15 => {}
        //~^ ERROR the range pattern here has ambiguous interpretation
        //~^^ HELP add parentheses to clarify the precedence
        box (16..=20) => {}
        _ => {}
    }
}
