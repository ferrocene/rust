trait I {}
type K = dyn I;
impl K for isize {} //~ ERROR expected trait, found type alias `K`

use crate::ImportError; //~ ERROR unresolved import `crate::ImportError` [E0432]
                        //~^ no `ImportError` in the root
impl ImportError for () {} // check that this is not an additional error (cf. issue #35142)

fn main() {}
