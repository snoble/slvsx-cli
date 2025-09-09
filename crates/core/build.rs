use std::env;
use std::path::PathBuf;

fn main() {
    // Only build FFI if not using mock solver
    #[cfg(not(feature = "mock-solver"))]
    {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_root = manifest_dir.parent().unwrap().parent().unwrap();

        // Compile the real SLVS wrapper
        // Support both the fork and the original submodule
        let (include_dir, src_dir) = if env::var("SLVS_USE_FORK").is_ok() {
            (project_root.join("libslvs-static/include"),
             project_root.join("libslvs-static/src"))
        } else {
            (project_root.join("libslvs/SolveSpaceLib/include"),
             project_root.join("libslvs/SolveSpaceLib"))
        };
        
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
            // Default path for local builds - use libslvs-static
            project_root.join("libslvs-static/build")
        };
        
        println!("cargo:rustc-link-search=native={}", slvs_lib_dir.display());
        
        // Link the static library and its dependencies
        // Always use the combined library from libslvs-static that includes mimalloc
        println!("cargo:rustc-link-lib=static=slvs-combined");
        
        // mimalloc is included in slvs-combined, no need to link separately

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
}
