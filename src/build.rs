use std::env;
use std::env::consts;
use std::path::Path;

fn main() {
    if !try_find_library() {
        compile_library_static();
    }
}

fn link_library_dynamic(lib_dir: &Path) {
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=cubature");
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
    }
}

fn link_library_static(lib_dir: &Path) {
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=cubature");
    // Link libm on Unix-like systems when building statically (Windows has no libm)
    if cfg!(not(target_os = "windows")) {
        println!("cargo:rustc-link-lib=m");
    }
}

fn try_find_library() -> bool {
    if let Ok(include_dir) = env::var("CUBATURE_INCLUDE_DIR") {
        println!("cargo:include={}", include_dir);
    }

    match env::var_os("CUBATURE_LIB_DIR") {
        Some(lib_dir) => {
            let lib_dir = Path::new(&lib_dir);
            let dylib_name = format!("{}cubature{}", consts::DLL_PREFIX, consts::DLL_SUFFIX);
            let static_candidates = ["libcubature.a", "cubature.lib"]; // .lib for MSVC static
            let dynamic_candidates = [&dylib_name[..], "cubature.so"]; // generic linux name fallback

            // Prefer a static library if present
            if static_candidates
                .iter()
                .any(|file| lib_dir.join(file).exists())
            {
                link_library_static(lib_dir);
                return true;
            }

            // Otherwise, accept a dynamic library
            if dynamic_candidates
                .iter()
                .any(|file| lib_dir.join(file).exists())
            {
                link_library_dynamic(lib_dir);
                return true;
            }

            println!(
                "cargo:warning=CUBATURE_LIB_DIR path ({}) did not contain {dylib_name}",
                lib_dir.display()
            );

            false
        }
        _ => false,
    }
}

fn compile_library_static() {
    // Expose headers to downstream crates
    println!(
        "cargo:include={}",
        dunce::canonicalize("vendor").unwrap().display()
    );

    // Compile the vendor sources into a static library via the cc crate.
    let mut build = cc::Build::new();
    build
        .cargo_metadata(true)
        .include("vendor")
        .file("vendor/hcubature.c")
        .file("vendor/pcubature.c");

    // Respect common C standards and warnings when supported
    build.flag_if_supported("-std=c11");
    build.warnings(true);

    // Produce a static library named "cubature"
    build.compile("cubature");

    // cc emits the link-search and -l: we just ensure libm on Unix for math functions
    if cfg!(not(target_os = "windows")) {
        println!("cargo:rustc-link-lib=m");
    }
}
