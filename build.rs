use std::env;
use std::path::PathBuf;

fn main() {
    build_from_source();
}

fn build_from_source() {
    println!("cargo:warning=Building TLSH from source");
    let target = env::var("TARGET").unwrap();
    let mut config = cc::Build::new();
    config.cpp(true);

    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    if target.contains("windows") {
        config.define("TLSH_EXPORTS", "1");
        config.define("TLSH_LIB", "1");
        config.define("WINDOWS", "1");
    } else {
        const TLSH_VERSION_SOURCE: &str = r#"
/****************************************************
 * This file is generated by cmake.  Modify the top
 * level CMakeLists.txt to change the VERSION numbers
 ****************************************************/

#define VERSION_MAJOR 4
#define VERSION_MINOR 8
#define VERSION_PATCH 2
#define TLSH_HASH "compact hash"
#define TLSH_CHECKSUM "1 byte checksum"
        "#;

        let gen_include_dir = dst.join("tlsh-4.8.2/include");
        std::fs::create_dir_all(&gen_include_dir).unwrap();
        std::fs::write(
            dst.join("tlsh-4.8.2/include/tlsh_version.h"),
            TLSH_VERSION_SOURCE,
        )
        .unwrap();
        config.include(gen_include_dir);
    }

    // default parameters
    config.define("TLSH_CHECKSUM_1B", "1");
    config.define("TLSH_BUCKETS_128", "1");
    config.define("BUCKETS_128", "1");
    config.define("TLSH_DISTANCE_PARAMETERS", "0");

    config
        .include("tlsh-4.8.2/include") //set(TLSH_SRCS tlsh.cpp tlsh_impl.cpp tlsh_util.cpp input_desc.cpp shared_file_functions.cpp)
        .include(".")
        .file("tlsh-4.8.2/src/tlsh.cpp")
        .file("tlsh-4.8.2/src/tlsh_impl.cpp")
        .file("tlsh-4.8.2/src/tlsh_util.cpp")
        .file("tlsh-4.8.2/src/input_desc.cpp")
        .file("tlsh-4.8.2/src/shared_file_functions.cpp")
        .file("tlsh_capi.cpp")
        .out_dir(dst.join("lib"))
        .compile("libtlsh.a");

    println!("cargo:rerun-if-changed=tlsh_capi.cpp");
    println!("cargo:rerun-if-changed=tlsh_capi.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=tlsh-4.8.2");

    println!("cargo:warning=TLSH lib: {}", dst.join("lib").display());
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=tlsh");
}
