use std::env;
use std::path::PathBuf;

fn main() {
    // Only build FFI if not using mock solver
    #[cfg(not(feature = "mock-solver"))]
    {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_root = manifest_dir.parent().unwrap().parent().unwrap();

        // Compile the real SLVS wrapper
        cc::Build::new()
            .file(project_root.join("ffi/real_slvs_wrapper.c"))
            .include(project_root.join("ffi"))
            .include(project_root.join("libslvs/SolveSpaceLib/include"))
            .include(project_root.join("libslvs/SolveSpaceLib"))
            .compile("real_slvs_wrapper");

        println!("cargo:rustc-link-lib=static=real_slvs_wrapper");
        
        // Check for SLVS_LIB_DIR environment variable (used in CI)
        let slvs_lib_dir = if let Ok(dir) = env::var("SLVS_LIB_DIR") {
            PathBuf::from(dir)
        } else {
            // Default path for local builds
            project_root.join("libslvs/SolveSpaceLib/build/src/slvs")
        };
        
        println!("cargo:rustc-link-search=native={}", slvs_lib_dir.display());
        
        // Also check common build directories
        println!(
            "cargo:rustc-link-search=native={}",
            project_root.join("libslvs/SolveSpaceLib/build/bin").display()
        );
        
        // Link the static library and its dependencies
        println!("cargo:rustc-link-lib=static=slvs");
        
        // Check for static build of mimalloc - only link if the library file exists
        let mimalloc_dir = project_root.join("libslvs/SolveSpaceLib/build/extlib/mimalloc");
        let mimalloc_lib = mimalloc_dir.join("libmimalloc.a");
        if mimalloc_lib.exists() {
            println!("cargo:rustc-link-search=native={}", mimalloc_dir.display());
            println!("cargo:rustc-link-lib=static=mimalloc");
        } else {
            // Try minimal build location
            let minimal_mimalloc = project_root.join("libslvs/SolveSpaceLib/build-minimal/libmimalloc.a");
            if minimal_mimalloc.exists() {
                println!("cargo:rustc-link-search=native={}", 
                    project_root.join("libslvs/SolveSpaceLib/build-minimal").display());
                println!("cargo:rustc-link-lib=static=mimalloc");
            }
            // If mimalloc is not found, we'll use system malloc (built into our minimal libslvs.a)
        }

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
