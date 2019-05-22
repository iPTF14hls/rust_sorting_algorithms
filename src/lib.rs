pub mod ms;
pub mod ss;
pub mod boso;
pub mod buso;
extern crate rand;

#[cfg(test)]
mod test_functions {
    pub fn sort_properly_sorts(sort: &Fn(&mut [usize]), len: usize) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        //First we create a sorted vec.
        let mut sort_vec: Vec<usize> = (0..len).collect();
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
