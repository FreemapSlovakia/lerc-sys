use std::env;
use std::path::PathBuf;

fn main() {
    let lerc_include = "vendor/lerc/src/LercLib/include";
    let lerc_src = "vendor/lerc/src/LercLib/Lerc_c_api_impl.cpp";

    // Build LERC C++ code
    cc::Build::new()
        .cpp(true)
        .include(lerc_include)
        .file(lerc_src)
        .compile("lerc");

    println!("cargo:rerun-if-changed={}", lerc_src);
    println!("cargo:rerun-if-changed={}/Lerc_c_api.h", lerc_include);
    println!("cargo:rerun-if-changed={}/Lerc_types.h", lerc_include);

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(format!("{}/Lerc_c_api.h", lerc_include))
        .clang_arg(format!("-I{}", lerc_include))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
