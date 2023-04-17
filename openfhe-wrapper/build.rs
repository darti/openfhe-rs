use std::{env, path::PathBuf};

use cmake::Config;

const OPENFHE_CORE: &str = "OPENFHEcore";
const OPENFHE_PKE: &str = "OPENFHEpke";
const OPENFHE_BIN: &str = "OPENFHEbinfhe";

fn build_wrapper() -> PathBuf {
    let mut cfg = Config::new("wrapper");

    if let Some(include) = std::env::var_os("DEP_OPENFHE_INCLUDE") {
        cfg.define("OpenFHE_INCLUDES", include);
    }

    for (key, value) in env::vars() {
        println!("{}: {:?}", key, value);
    }

    cfg.build()
}

fn generate_binding(openfhe_root: &str) {
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .blocklist_file(" /Users/matthieudartiguenave/Projects/etf/openfhe-rs/target/debug/build/openfhe-sys-f494f1d17fc0f95b/out/include/openfhe/core/math/**/*")
        .header("wrapper/src/wrapper.hpp")
        .clang_arg("-Fpath/to/dir")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-std=c++17")
        .clang_arg("-D MATHBACKEND=4")
        .opaque_type("std::*")
        .clang_args([
            format!("-F{}/include/openfhe", openfhe_root),
            format!("-F{}/include/openfhe/core", openfhe_root),
            format!("-F{}/include/openfhe/pke", openfhe_root),
            format!("-F{}/include/openfhe/binfhe", openfhe_root),
        ])
        // allowlist
        .allowlist_file(".*wrapper.hpp")
        .allowlist_file(".*parameters.hpp")
        .allowlist_file(".*context.hpp")
        .allowlist_file(".*scheme/.*")
        .allowlist_file(".*pke/constants\\.h")
        .allowlist_file(".*pke/.*cryptocontext.*\\.h")
        .allowlist_file(".*utils/.*")
        // blacklist
        // .blocklist_file(".*math/.*")
        // .blocklist_file(".*lattice/.*")
        // .blocklist_file(".*utils/.*")
        .blocklist_file(".*cereal/.*")
        // .blocklist_file(".*lbcrypto/.*")
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

    let openfhe_root = std::env::var_os("DEP_OPENFHE_ROOT").unwrap();
    let openfhe_root = openfhe_root.to_str().unwrap();

    println!("cargo:rustc-link-search={}/lib", openfhe_root);

    // println!("cargo:rustc-link-lib=static=openfhe-wrapper");
    println!("cargo:rustc-flags=-l dylib=c++");

    // let dst = build_wrapper();

    // println!("cargo:rustc-link-search={}", dst.display());
    generate_binding(openfhe_root);

    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_CORE);
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_PKE);
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_BIN);

    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/libomp/lib");
    println!("cargo:rustc-link-lib=static={}", "omp");
}
