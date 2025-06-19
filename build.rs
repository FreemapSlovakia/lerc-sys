use glob::glob;
use std::{env, path::PathBuf};

fn main() {
    build_lerc();
    generate_bindings("vendor/lerc/src/LercLib/include");
}

fn build_lerc() {
    let base = "vendor/lerc/src/LercLib";

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .include(format!("{base}/include"))
        .include(base);

    for entry in glob(&format!("{base}/**/*.cpp")).expect("Failed to read glob pattern") {
        let path = entry.expect("Invalid .cpp path");
        build.file(path);
    }

    build.compile("lerc");

    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={base}");
}

fn generate_bindings(include_path: &str) {
    let bindings = bindgen::Builder::default()
        .header(format!("{}/Lerc_c_api.h", include_path))
        .clang_arg(format!("-I{}", include_path))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
