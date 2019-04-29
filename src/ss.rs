use std::{
    cmp::{Ord, Ordering},
    mem::swap,
};

#[test]
fn selection_sort_properly_sorts() {
    use crate::test_functions::sort_properly_sorts;
    sort_properly_sorts(&selection_sort)
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
