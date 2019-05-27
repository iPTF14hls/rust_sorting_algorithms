use std::cmp::{Ord, Ordering};

#[bench]
fn merge_sort_properly_sorts(b: &mut test::Bencher) {
    use crate::test_functions::sort_testing;
    sort_testing(&merge_sort, 10_000, b)
}

fn merge<T: Ord + Copy>(mut lef: &[T], mut rig: &[T]) -> Vec<T> {
    let mut temp = Vec::with_capacity(lef.len() + rig.len());
    
    while !lef.is_empty() && !rig.is_empty() {
        match lef[0].cmp(&rig[0]) {
            Ordering::Less | Ordering::Equal => {
                temp.push(lef[0]);
                lef = &lef[1..];
            },
            Ordering::Greater => {
                temp.push(rig[0]);
                rig = &rig[1..];
            }
        }
    }
    
    temp.extend_from_slice(lef);
    temp.extend_from_slice(rig);

    temp
}

pub fn merge_sort<T: Ord + Copy>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }
    let (lef, rig) = array.split_at_mut(len/2);
    
    merge_sort(lef);
    merge_sort(rig);
    
    let temp = merge(lef, rig);
    
    for (i, val) in temp.into_iter().enumerate() {
        array[i] = val;
    }
}
