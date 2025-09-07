use std::env;
use std::path::PathBuf;

fn main() {
    // Compile the C wrapper
    cc::Build::new()
        .file("slvs_wrapper.c")
        .compile("slvs_wrapper");
    
    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("slvs_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}