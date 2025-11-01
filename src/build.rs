use std::env;
use std::env::consts;
use std::path::Path;

fn main() {
    if !try_find_library() {
        compile_library();
    }
}

fn link_library(lib_dir: &Path) {
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=cubature");
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
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
                link_library(lib_dir);
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

    let lib_output_dir = format!("$<1:{}>", env::var("OUT_DIR").unwrap());
    let mut config = cmake::Config::new("vendor");
    config
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .define("CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS", "ON")
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY", &lib_output_dir)
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", &lib_output_dir)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", &lib_output_dir)
        .build_target("cubature");
    let dst = config.build();
    link_library(&dst);
}
