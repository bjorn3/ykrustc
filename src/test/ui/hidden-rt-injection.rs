// This is testing that users can't access the runtime crate.

mod m {
    // The rt has been called both 'native' and 'rt'
    use native; //~ ERROR unresolved import
}

fn main() { }
