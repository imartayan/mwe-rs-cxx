fn build_tiered_vec() {
    println!("cargo:rerun-if-changed=src/tiered_vec.rs");
    println!("cargo:rerun-if-changed=cxx/tiered_vec.cc");
    println!("cargo:rerun-if-changed=cxx/tiered_vec.h");

    cxx_build::bridge("src/tiered_vec.rs")
        .file("cxx/tiered_vec.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("cxxbridge_tiered_vec");
}

fn build_rank_bv() {
    println!("cargo:rerun-if-changed=src/rank_bv.rs");
    println!("cargo:rerun-if-changed=cxx/rank_bv.cc");
    println!("cargo:rerun-if-changed=cxx/rank_bv.h");

    cxx_build::bridge("src/rank_bv.rs")
        .file("cxx/rank_bv.cc")
        .flag_if_supported("-std=c++17")
        .compile("cxxbridge_rank_bv");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_tiered_vec();
    build_rank_bv();
}
