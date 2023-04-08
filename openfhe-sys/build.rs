use std::env;

use cmake::Config;

// Options

const OPENFHE_CORE: &str = "OPENFHEcore";
const _OPENFHE_PKE: &str = "OPENFHEpke";
const _OPENFHE_BIN: &str = "OPENFHEbinfhe";

fn main() {
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

    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static={}_static", OPENFHE_CORE);
}
