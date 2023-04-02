use std::{env, path::PathBuf};

use cmake::Config;

// Options

const OPENFHE_CORE: &str = "OPENFHEcore";
const OPENFHE_PKE: &str = "OPENFHEpke";
const OPENFHE_BIN: &str = "OPENFHEbinfhe";

//INSTALL_INCLUDE_DIR:PATH=include/openfhe

fn main() {
    let dst = Config::new("openfhe-development")
        .define("BUILD_STATIC", "ON")
        .define("BUILD_SHARED", "OFF")
        .define("BUILD_BENCHMARKS", "OFF")
        .define("BUILD_UNITTESTS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        // .define("INSTALL_INCLUDE_DIR", "include")
        .build();

    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_CORE);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include/openfhe/core", dst.display()))
        .clang_arg(format!("-I{}/include/openfhe", dst.display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
