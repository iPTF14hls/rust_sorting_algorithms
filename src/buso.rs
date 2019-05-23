use std::cmp::{Ord, Ordering};

#[test]
fn bubble_sort_properly_sorts() {
    use crate::test_functions::sort_properly_sorts;
    sort_properly_sorts(&bubble_sort, 10_000)
}

pub fn bubble_sort<T: Ord + Copy>(mut array: &mut [T]) {
    while array.len() > 1 {
        let len = array.len();
        let (mut i, mut ip1) = (0, 1);

        while array.len() > ip1 {
            if let Ordering::Less = array[ip1].cmp(&array[i]) {
                array.swap(i, ip1);
            }

            i+=1;
            ip1+=1;
        }

        array = &mut array[..len-1];
    }
}
