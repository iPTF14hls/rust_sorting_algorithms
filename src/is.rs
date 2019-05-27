use std::cmp::{Ord, Ordering};

#[bench]
fn insertion_sort_properly_sorts(b: &mut test::Bencher) {
    use crate::test_functions::{sort_testing, BENCH_CONSTANT};
    sort_testing(&insertion_sort, BENCH_CONSTANT, b)
}

pub fn insertion_sort<T: Ord + Copy>(array: &mut [T]) {
    for i in 1..array.len() {
        for j in (1..=i).rev() {
            if array[j].cmp(&array[j - 1]) == Ordering::Less {
                array.swap(j, j - 1);
            } else {
                break;
            }
        }
    }
}
