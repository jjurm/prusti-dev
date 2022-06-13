use prusti_contracts::*;

use std::option::Option;

// Task: allow importing the following specification
#[extern_spec]
impl<T> Option<T> {
    #[pure]
    #[ensures(matches!(*self, Some(_)) == result)]
    fn is_some(&self) -> bool;
}


fn main() {
    let a = Some(42);
    let b = None::<i32>;


    assert!(a.is_some() == true);
    assert!(b.is_some() == false);

}
