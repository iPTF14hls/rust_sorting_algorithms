use std::{
    cmp::{Ord, Ordering},
    mem::swap,
};

#[bench]
fn selection_sort_properly_sorts(b: &mut test::Bencher) {
    use crate::test_functions::sort_testing;
    sort_testing(&selection_sort, 10_000, b)
}

pub fn selection_sort<T: Ord + Copy>(array: &mut [T]) {
    for i in 1..array.len() {
        let (zero, ass) = array.split_at_mut(i);

        for v in ass.iter_mut() {
            if let Ordering::Greater = zero[zero.len() - 1].cmp(v) {
                swap(&mut (*v), &mut zero[zero.len() - 1])
            }
        }
    }
}
