#![allow(dead_code)]

pub use cxx::UniquePtr;
use std::fmt::Debug;

#[cxx::bridge]
mod tv28 {
    unsafe extern "C++" {
        include!("cbl/cxx/tiered_vec.h");

        type TieredVec28;
        fn new_tiered_vec_28() -> UniquePtr<TieredVec28>;
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool;
        fn capacity(&self) -> usize;
        fn get(&self, idx: usize) -> u32;
        fn update(&self, idx: usize, elem: u32) -> u32;
        fn insert(&self, idx: usize, elem: u32);
        fn remove(&self, idx: usize);
        fn insert_sorted(&self, elem: u32);
        fn contains_sorted(&self, elem: u32) -> bool;
        fn index_sorted(&self, elem: u32) -> usize;
    }
}
pub use tv28::*;

impl Debug for TieredVec28 {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
