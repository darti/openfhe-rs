use std::{env, path::PathBuf};

use cmake::Config;

fn build_wrapper() -> PathBuf {
    let mut cfg = Config::new("wrapper");

    if let Some(include) = std::env::var_os("DEP_OPENFHE_INCLUDE") {
        cfg.define("OpenFHE_INCLUDES", include);
    }

    for (key, value) in env::vars() {
        println!("{}: {:?}", key, value);
    }

    println!("DEP_Z_INCLUDE: {:?}", std::env::var_os("DEP_Z_INCLUDE"));

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
    let _dst = build_wrapper();
    generate_binding();
}
