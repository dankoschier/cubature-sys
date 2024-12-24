use std::env;
use std::env::consts;
use std::path::Path;

fn main() {
    if !try_find_library() {
        compile_library();
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
            if [&dylib_name, "libcubature.a", "cubature.so", "cubature.lib"]
                .iter()
                .any(|file| lib_dir.join(file).exists())
            {
                println!("cargo:rustc-link-search=native={}", lib_dir.display());
                println!("cargo:rustc-link-lib=cubature");
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

fn compile_library() {
    println!(
        "cargo:include={}",
        dunce::canonicalize("vendor").unwrap().display()
    );

    // Don't use the CMakeLists.txt as this requires the gigantic clencurt.h to be present.
    // Due to the size limitations of crates.io we would not be able to submit the package with this
    // file. Therefore, we use cc instead and use only what we need.
    let mut build = cc::Build::new();
    build
        .include("vendor")
        .file("vendor/hcubature.c")
        .file("vendor/pcubature.c")
        .flag_if_supported("-Wno-sign-compare");
    if env::var_os("DEBUG").as_deref() != Some("true".as_ref()) {
        build.define("NDEBUG", Some("1"));
    }
    build.compile("cubature");

    println!("cargo:rustc-link-lib=cubature");
}
