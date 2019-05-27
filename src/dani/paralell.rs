use std::cmp::Ordering;
//Lines 1 and 2 are for test featuries.
//Ordering is used for the merge function.
//rand uses SliceRandom and thread_rng to shuffle the initial vector.
#[bench]
fn dani_merge_sort_paralell_properly_sorts(b: &mut test::Bencher) {
    use crate::test_functions::{sort_testing, BENCH_CONSTANT};
    sort_testing(
        &|array: &mut [usize]| {
            let mut arr_vec = Vec::new();
            array.clone_into(&mut arr_vec);
            let sort_vec = merge_sort(arr_vec);
            array.iter_mut().zip(sort_vec.iter()).for_each(|(o, e)| {
                *o = *e;
            });
        },
        BENCH_CONSTANT,
        b,
    )
}

pub fn merge_sort<T: PartialOrd + Copy + Send + Sync>(sort_vec: Vec<T>) -> Vec<T> {
    if sort_vec.len() > 2 {
        let (vec_a, vec_b) = rayon::join(
            || merge_sort(sort_vec[0..sort_vec.len() / 2].to_vec()),
            || merge_sort(sort_vec[sort_vec.len() / 2..sort_vec.len()].to_vec()),
        );
        merge(vec_a, vec_b)
    //Creates sub-vectors containing the rescursively-sorted first and second halves of the input.
    //Returns merged product of sub-vectors.
    } else {
        if sort_vec.len() == 1 || sort_vec[0] < sort_vec[1] {
            return sort_vec;
        }
        vec![sort_vec[1], sort_vec[0]]
        //Bottom level of recursion when vector len <= 2,
        //returns self if ordered, returns swapped values otherwise.
    }
}

fn merge<T: PartialOrd + Copy>(vec_1: Vec<T>, vec_2: Vec<T>) -> Vec<T> {
    let mut vec_merged: Vec<T> = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    //Creates output buffer and 2 indices for merging.
    while left_index < vec_1.len() && right_index < vec_2.len() {
        match vec_1[left_index].partial_cmp(&vec_2[right_index]) {
            Some(Ordering::Less) | Some(Ordering::Equal) => {
                vec_merged.push(vec_1[left_index]);
                left_index += 1;
            }
            Some(Ordering::Greater) => {
                vec_merged.push(vec_2[right_index]);
                right_index += 1;
            }
            None => panic!("Tried to sort NaN value"),
        }
    }

    vec_merged.extend_from_slice(&vec_1[left_index..vec_1.len()]);
    vec_merged.extend_from_slice(&vec_2[right_index..vec_2.len()]);
    vec_merged
}
