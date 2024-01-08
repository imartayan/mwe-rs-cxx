// #![allow(incomplete_features)]
// #![feature(slice_group_by)]
// #![feature(generic_const_exprs)]

// mod bit_container;
// mod cbl;
// mod compact_int;
// mod container;
// mod kmer;
// mod necklace;
mod rank_bv;
// mod reads;
mod tiered_vec;
// mod wordset;

// use cbl::*;

fn main() {
    println!("Hi ffi!");

    let rbv = rank_bv::new_rank_bv(20);
    rbv.set(12);
    assert!(rbv.get(12));

    let tv = tiered_vec::new_tiered_vec_28();
    tv.insert(0, 42);
    assert_eq!(tv.get(0), 42);

    println!("All set!");
}
