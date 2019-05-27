use std::cmp::{Ord, Ordering};

#[bench]
fn quick_sort_properly_sorts(b: &mut test::Bencher) {
    use crate::test_functions::sort_testing;
    sort_testing(&quick_sort, 10_000, b)
}

pub fn quick_sort<T: Ord + Copy>(array: &mut [T])
{
    match array.len() {
        0 | 1 => {}
        2 => {
            if array[1].cmp(&array[0]) == Ordering::Less {
                array.swap(0, 1);
            }
        }
        _ => {
            let mut piv_index = 0;

            //Here we do the pivot and partition steps of quick sort.
            //The sorting algorithm itself dosen't need to 
            {
                let (piv, arr) = {
                    let (l, r) = array.split_at_mut(1);
                    (&mut l[0], r)
                };

                for i in 0..arr.len() {
                    if arr[i].cmp(piv) == Ordering::Less {
                        arr.swap(piv_index, i);
                        piv_index += 1;
                    }
                }
            }
            
            array.swap(0, piv_index);

            let (lef, rig) = array.split_at_mut(piv_index);
            let rig = &mut rig[1..];

            quick_sort(lef);
            quick_sort(rig);
        }
    }
}
