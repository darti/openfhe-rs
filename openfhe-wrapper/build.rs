use std::{env, path::PathBuf};

use cmake::Config;

const OPENFHE_CORE: &str = "OPENFHEcore";
const OPENFHE_PKE: &str = "OPENFHEpke";
const OPENFHE_BIN: &str = "OPENFHEbinfhe";

fn build_wrapper() -> PathBuf {
    for (key, value) in env::vars() {
        println!("{}: {:?}", key, value);
    }
    let mut cfg = Config::new("wrapper");

    if let Some(include) = std::env::var_os("DEP_OPENFHE_INCLUDE") {
        cfg.define("OpenFHE_INCLUDES", include);
    }

    for (key, value) in env::vars() {
        println!("{}: {:?}", key, value);
    }

    cfg.build()
}

fn generate_binding() {
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper/src/wrapper.hpp")
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
    println!("cargo:rerun-if-changed=wrapper.h");

    if let Some(lib) = std::env::var_os("DEP_OPENFHE_ROOT") {
        println!("cargo:rustc-link-search={}/lib", lib.to_str().unwrap());
    }

    println!("cargo:rustc-link-lib=static=openfhe-wrapper");
    println!("cargo:rustc-flags=-l dylib=c++");

    let dst = build_wrapper();

    println!("cargo:rustc-link-search={}", dst.display());
    generate_binding();

    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_CORE);
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_PKE);
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_BIN);

    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/libomp/lib");
    println!("cargo:rustc-link-lib=static={}", "omp");
}
