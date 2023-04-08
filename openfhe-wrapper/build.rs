use std::env;

use cmake::Config;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut cfg = Config::new("wrapper");

    if let Some(include) = std::env::var_os("DEP_OPENFHE_INCLUDE") {
        cfg.define("OpenFHE_INCLUDES", include);
    }

    for (key, value) in env::vars() {
        println!("{}: {:?}", key, value);
    }

    println!("DEP_Z_INCLUDE: {:?}", std::env::var_os("DEP_Z_INCLUDE"));

    cfg.build();
}
