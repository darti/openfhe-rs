use std::env;

use cmake::Config;

// Options

const OPENFHE_CORE: &str = "OPENFHEcore";
const OPENFHE_PKE: &str = "OPENFHEpke";
const OPENFHE_BIN: &str = "OPENFHEbinfhe";

fn main() {
    for (key, value) in env::vars() {
        println!("{}: {:?}", key, value);
    }

    let dst = Config::new("openfhe")
        .define("BUILD_STATIC", "ON")
        .define("BUILD_SHARED", "OFF")
        .define("BUILD_BENCHMARKS", "OFF")
        .define("BUILD_UNITTESTS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define(
            "INSTALL_INCLUDE_DIR",
            format!("{}/include", env::var("CARGO_MANIFEST_DIR").unwrap()),
        )
        .build();

    println!("cargo:include={}/include/openfhe", dst.to_str().unwrap());

    println!("cargo:rustc-link-search={}/build/lib", dst.display());

    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_CORE);
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_PKE);
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_BIN);
    println!("cargo:rustc-link-lib=static={}", "omp");

    println!("cargo:rustc-flags=-l dylib=c++");
}
