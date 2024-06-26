use std::{env::var, path::PathBuf};

pub fn main() {
    println!("cargo:rustc-link-search=../csp_channelizer");
    println!("cargo:rustc-link-lib=cspch");
    println!("cargo:rustc-link-lib=fftw3f");
    println!("cargo:rustc-link-search=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-lib=cufft");
    println!("cargo:rustc-link-lib=stdc++");

    let header = PathBuf::from("../csp_channelizer/cspch.h");
    println!(
        "cargo:rerun-if-changed={}",
            header.to_str().expect("invalid path")
    );
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header.to_str().expect("invalid path"))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
