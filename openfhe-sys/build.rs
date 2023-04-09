use std::{env, path::PathBuf};

use cmake::Config;

// Options

const OPENFHE_CORE: &str = "OPENFHEcore";
const OPENFHE_PKE: &str = "OPENFHEpke";
const OPENFHE_BIN: &str = "OPENFHEbinfhe";

fn main() {
    // for (key, value) in env::vars() {
    //     println!("{}: {:?}", key, value);
    // }

    let dst_openfe = build_openfhe();

    println!(
        "cargo:include={}/include/openfhe",
        dst_openfe.to_str().unwrap()
    );

    let _dst_wrapper = build_wrapper(dst_openfe.clone());

    generate_binding();

    linking(dst_openfe);
}

fn linking(dst_openfe: PathBuf) {
    println!("cargo:rustc-link-search={}/lib", dst_openfe.display());

    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_CORE);
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_PKE);
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_BIN);
    println!("cargo:rustc-link-lib=static={}", "omp");

    println!("cargo:rustc-flags=-l dylib=c++");
}

fn build_openfhe() -> PathBuf {
    Config::new("openfhe")
        .define("BUILD_STATIC", "ON")
        .define("BUILD_SHARED", "OFF")
        .define("BUILD_BENCHMARKS", "OFF")
        .define("BUILD_UNITTESTS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define(
            "INSTALL_INCLUDE_DIR",
            format!("{}/include", env::var("CARGO_MANIFEST_DIR").unwrap()),
        )
        .build()
}

fn build_wrapper(openfhe_dir: PathBuf) -> PathBuf {
    let mut cfg = Config::new("wrapper");

    cfg.define(
        "OpenFHE_INCLUDES",
        openfhe_dir.join("include").join("openfhe"),
    );

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
