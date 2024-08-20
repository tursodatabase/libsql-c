use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=./libsql.h");

    let bindings = bindgen::Builder::default()
        .header("libsql.h")
        .rustified_enum("libsql_type_t")
        .rustified_enum("libsql_cypher_t")
        .rustified_enum("libsql_tracing_level_t")
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
