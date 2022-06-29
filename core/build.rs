extern crate bindgen;

use std::env;
use std::path::PathBuf;

// Stolen from @lvaccaro implementation
fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    let root_path = PathBuf::from(env::current_dir().unwrap());
    let lib_path = format!("{}/include/lnsocket/", root_path.display());
    println!("cargo:rustc-link-search={}", lib_path);

    // Tell cargo to tell rustc to link the shared library.
    println!("cargo:rustc-link-lib=lnsocket");
    println!("cargo:rustc-link-lib=secp256k1");
    println!("cargo:rustc-link-lib=sodium");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/lnsocket.h", lib_path))
        .header(format!("{}/lnsocket_internal.h", lib_path))
        .clang_arg(format!("-I{}/deps/secp256k1/include/", lib_path))
        .header(format!("{}/deps/secp256k1/include/secp256k1.h", lib_path))
        .trust_clang_mangling(false)
        .blacklist_item("IPPORT_RESERVED")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("Generating binding on {}", out_path.display());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
