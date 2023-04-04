use std::env;

fn main() {
    println!("cargo:rustc-link-search=native=../src");
    println!("cargo:rustc-link-search=native=../deps/lua/src");
    println!("cargo:rustc-link-search=native=../deps/hdr_histogram");
    println!("cargo:rustc-link-search=native=../deps/fpconv");
    println!("cargo:rustc-link-search=native=../deps/jemalloc/lib");
    println!("cargo:rustc-link-search=native=../deps/hiredis");
    println!("cargo:rustc-link-lib=static=hiredis");
    println!("cargo:rustc-link-lib=static=lua");
    println!("cargo:rustc-link-lib=static=hdrhistogram");
    println!("cargo:rustc-link-lib=static=fpconv");
    println!("cargo:rustc-link-lib=static=jemalloc");
    println!("cargo:rustc-link-lib=static=redis_server");
}
