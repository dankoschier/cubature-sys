use std::env;
use std::env::consts;
use std::path::Path;

fn main() {
    if let Ok(include_dir) = env::var("CUBATURE_INCLUDE_DIR") {
        println!("cargo:include={}", include_dir);
    }

    let mut found_target = false;
    if let Some(lib_dir) = env::var_os("CUBATURE_LIB_DIR") {
        let lib_dir = Path::new(&lib_dir);
        let dylib_name = format!("{}cubature{}", consts::DLL_PREFIX, consts::DLL_SUFFIX);
        if [&dylib_name, "libcubature.a", "cubature.so", "cubature.lib"]
            .iter()
            .any(|file| lib_dir.join(file).exists())
        {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=cubature");
            found_target = true;
        } else {
            println!(
                "cargo:warning=CUBATURE_LIB_DIR path ({}) did not contain {dylib_name}",
                lib_dir.display()
            );

            if cfg!(feature = "static-fallback") {
                println!(
                    "cargo:warning=Will attempt to build cubature library and link statically"
                );
            }
        }
    }

    if !found_target && cfg!(feature = "static-fallback") {
        compile_static();
    }
}

#[cfg(not(any(feature = "static", feature = "static-fallback")))]
fn compile_static() {
    println!("cargo:warning='static' feature of cubature-sys is disabled, so the library won't be built, and probably won't work at all");
    println!("cargo:rustc-link-lib=cubature");
}

#[cfg(any(feature = "static", feature = "static-fallback"))]
fn compile_static() {
    let mut cc = cc::Build::new();
    cc.include("vendor")
        .file("vendor/hcubature.c")
        .file("vendor/pcubature.c")
        .flag_if_supported("-Wno-sign-compare");

    if env::var_os("DEBUG").as_deref() != Some("true".as_ref()) {
        cc.define("NDEBUG", Some("1"));
    }

    cc.compile("libcubature.a");
    println!(
        "cargo:include={}",
        dunce::canonicalize("vendor").unwrap().display()
    );
}
