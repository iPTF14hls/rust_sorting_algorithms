use std::cmp::{Ord, Ordering};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[test]
fn bogo_sort_properly_sorts() {
    use crate::test_functions::sort_properly_sorts;
    sort_properly_sorts(&bogo_sort, 8)
}

pub fn bogo_sort<T: Ord>(array: &mut [T]) {
    let mut rng = thread_rng();

    loop {

        let mut aiter1 = array.iter();
        aiter1.next();
        if array.iter().zip(aiter1).all(|(a, b)| match b.cmp(a) {
            Ordering::Equal | Ordering::Greater => true,
            Ordering::Less => false,
        }) {
            break;
        }

        array.shuffle(&mut rng);
    }
}
