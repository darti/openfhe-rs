use std::{env, path::PathBuf};

use cmake::Config;

// Options

const OPENFHE_CORE: &str = "OPENFHEcore";
const OPENFHE_PKE: &str = "OPENFHEpke";
const OPENFHE_BIN: &str = "OPENFHEbinfhe";

fn build_openfhe() -> PathBuf {
    let dst = Config::new("openfhe")
        .define("BUILD_STATIC", "ON")
        .define("BUILD_SHARED", "OFF")
        .define("BUILD_BENCHMARKS", "OFF")
        .define("BUILD_UNITTESTS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        // .define("INSTALL_INCLUDE_DIR", "include")
        .build();

    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_CORE);

    dst
}

fn build_wrapper(dst: PathBuf) -> PathBuf {
    let dst = Config::new("openfhe-wrapper").build();

    dst
}

fn run_bindgen(dst: PathBuf) {
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

fn main() {
    let dst_openfhe = build_openfhe();
    let _dst_wrapper = build_wrapper(dst_openfhe);
}
