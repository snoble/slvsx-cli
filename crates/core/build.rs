use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project_root = manifest_dir.parent().unwrap().parent().unwrap();

    // Only use the libslvs-static fork
    let include_dir = project_root.join("libslvs-static/include");
    let src_dir = project_root.join("libslvs-static/src");
    
    // Compile the real SLVS wrapper
    cc::Build::new()
        .file(project_root.join("ffi/real_slvs_wrapper.c"))
        .include(project_root.join("ffi"))
        .include(include_dir)
        .include(src_dir)
        .compile("real_slvs_wrapper");

    println!("cargo:rustc-link-lib=static=real_slvs_wrapper");
    
    // Check for SLVS_LIB_DIR environment variable (used in CI)
    let slvs_lib_dir = if let Ok(dir) = env::var("SLVS_LIB_DIR") {
        PathBuf::from(dir)
    } else {
        // Default path for local builds
        project_root.join("libslvs-static/build")
    };
    
    println!("cargo:rustc-link-search=native={}", slvs_lib_dir.display());
    
    // Link the static library
    println!("cargo:rustc-link-lib=static=slvs-combined");
    
    // System libraries needed by libslvs
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=m");
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
    }
}