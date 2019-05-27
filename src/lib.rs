#![feature(test, toowned_clone_into)]
extern crate test;

pub mod boso;
pub mod buso;
pub mod ms;
pub mod ss;
pub mod qs;
pub mod ts;
pub mod is;
pub mod dani;

extern crate rand;

#[cfg(test)]
mod test_functions {
    use test::Bencher;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    pub fn sort_testing(sort: &Fn(&mut [usize]), len: usize, b: &mut Bencher) {

        //First we create a sorted vec.
        let mut sort_vec: Vec<usize> = (0..len).collect();
        let orig = sort_vec.clone();

        b.iter(|| {
            sort_vec.shuffle(&mut thread_rng());
            let r = sort_vec.as_mut_slice();

            sort(r);

            assert!(
                orig.clone().into_iter().zip(r.iter_mut()).all(|(a, b)| a == *b),
                "{:?}",
                r
            )
        });
    }

    #[bench]
    fn rust_stable_sort(b: &mut test::Bencher) {
        use crate::test_functions::sort_testing;
        sort_testing(&|array: &mut [usize]| {
            array.sort();
        }, 10_000, b)
    }

    #[bench]
    fn rust_unstable_sort(b: &mut test::Bencher) {
        use crate::test_functions::sort_testing;
        sort_testing(&|array: &mut [usize]| {
            array.sort_unstable()
        }, 10_000, b)
    }
}
