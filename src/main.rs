mod ms;
mod ss;
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[cfg(test)]
mod test_functions {
    pub fn sort_properly_sorts(sort: &Fn(&mut [i32])) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        //First we create a sorted vec.
        let mut sort_vec: Vec<i32> = (0..100_000).collect();
        let orig = sort_vec.clone();
        
        sort_vec.shuffle(&mut thread_rng());
        let r = sort_vec.as_mut_slice();

        sort(r);

        assert!(orig
            .into_iter()
            .zip(r.iter_mut())
            .all(|(a, b)|a==*b))
    }
}

fn main() {
    let mut sort_vec: Vec<i32> = (0..100).collect();
    sort_vec.shuffle(&mut thread_rng());
    let r = sort_vec.as_mut_slice();
    println!("{:?}", r);
    crate::ss::selection_sort(r);

    println!("{:?}", r);
}
