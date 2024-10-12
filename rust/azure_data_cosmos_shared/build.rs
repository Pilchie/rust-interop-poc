fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");

    let header_name = std::env::var("CARGO_PKG_NAME").expect("'CARGO_PKG_NAME' env var should be set.") + ".h";
    let out_file = std::path::Path::new("..")
        .join("..")
        .join("target")
        .join(header_name);

    cbindgen::Builder::new()
    .with_crate(".")
    .with_language(cbindgen::Language::C)
    .generate()
    .expect("Unable to generate C bindings")
    .write_to_file(out_file);
}
