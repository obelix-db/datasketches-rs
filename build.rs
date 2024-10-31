use std::env;
use std::fs;
use std::path::PathBuf;

fn bindgen_datasketches_cpp() {
    let bindings = bindgen::Builder::default()
        .header("datasketches-cpp/theta/include/theta_union.hpp")
        .header("datasketches-cpp/theta/include/theta_intersection.hpp")
        .header("datasketches-cpp/theta/include/theta_a_not_b.hpp")
        .header("datasketches-cpp/theta/include/theta_jaccard_similarity.hpp")
        .header("datasketches-cpp/theta/include/theta_sketch.hpp")
        .clang_arg("-Idatasketches-cpp/common/include")
        .clang_arg("-Idatasketches-cpp/theta/include")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .opaque_type("std::.*")
        .enable_cxx_namespaces()
        .respect_cxx_access_specs(true)
        .opaque_type("datasketches::.*")
        .allowlist_type("datasketches::.*")
        .allowlist_function(".*")
        .allowlist_recursively(true)
        .derive_debug(false)
        .blocklist_type("max_align_t") // https://github.com/rust-lang-nursery/rust-bindgen/issues/550
        //.blocklist_type("std::.*")
        // .ctypes_prefix("libc")
        .size_t_is_usize(true)
        .generate()
        .expect("unable to generate rocksdb bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write rocksdb bindings");
}

fn main() {
    // TODO: generate bindgen bindings...
    bindgen_datasketches_cpp();

    // TODO: link datasketches

    // cxx_build::bridge("src/lib.rs")
    //     .file("src/blobstore.cc")
    //     .flag_if_supported("-std=c++14")
    //     .compile("datasketches-rs");
    //
    // println!("cargo:rerun-if-changed=src/lib.rs");
    // println!("cargo:rerun-if-changed=src/ffi.rs");
    // println!("cargo:rerun-if-changed=src/blobstore.cc");
    // println!("cargo:rerun-if-changed=include/blobstore.h");
}