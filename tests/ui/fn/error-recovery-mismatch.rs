//@ revisions: edition2015 edition2018
//@ [edition2015] edition: 2015
//@ [edition2018] edition: 2018..
// Used to ICE due to a size mismatch between the actual fake signature of `fold` and the
// generated signature used reporting the parameter mismatch at the call site.
// See issue #135124

trait A  {
    fn y(&self)
    {
        fn call() -> impl Sized {}
        self.fold(call(), call());
    }
    fn fold<T>(&self, _: T, &self._) {}
    //[edition2018]~^ ERROR expected one of `:` or `|`, found `)`
    //~^^ ERROR unexpected `self` parameter in function
    //~| ERROR expected one of `)` or `,`, found `.`
    //~| ERROR identifier `self` is bound more than once in this parameter list
    //[edition2015]~| WARNING anonymous parameters are deprecated
    //[edition2015]~| WARNING this is accepted in the current edition
    //[edition2015]~| ERROR the placeholder `_` is not allowed within types
}

fn main() {}
