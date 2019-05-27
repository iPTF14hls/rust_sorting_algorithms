use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{Ord, Ordering};

#[bench]
fn bogo_sort_properly_sorts(b: &mut test::Bencher) {
    use crate::test_functions::sort_testing;
    //Note, Array size cannot go any higher as the tests take awhile to finish.
    sort_testing(&bogo_sort, 8, b)
}

pub fn bogo_sort<T: Ord>(array: &mut [T]) {
    let mut rng = thread_rng();

    loop {
        let mut aiter1 = array.iter();
        aiter1.next();

        if array
            .iter()
            .zip(aiter1)
            .all(|(a, b)| a.cmp(b) == Ordering::Less)
        {
            break;
        }

        array.shuffle(&mut rng);
    }
}
