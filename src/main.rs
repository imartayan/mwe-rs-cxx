#![allow(unused_imports)]

mod rank_bv;
mod tiered_vec;

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
