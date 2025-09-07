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
            .include(project_root.join("SolveSpaceLib/libslvs/include"))
            .compile("real_slvs_wrapper");
        
        println!("cargo:rustc-link-lib=static=real_slvs_wrapper");
        println!("cargo:rustc-link-search=native={}", 
                 project_root.join("SolveSpaceLib/build/libslvs").display());
        println!("cargo:rustc-link-lib=dylib=libslvs");
    }
}