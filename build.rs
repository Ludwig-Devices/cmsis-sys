use std::env;
use std::path::PathBuf;

fn is_crosscompile() -> bool {
    env::var("HOST").unwrap() != env::var("TARGET").unwrap()
}

fn main() {
    let build_path = build();
    bindgen(&build_path);
}

fn bindgen(build_path: &PathBuf) {
    // Invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Only generate bindings for the arm_ functions and variables
        .allowlist_function("arm_.*")
        .allowlist_var("arm_.*")
        // Where to look for headers
        .clang_arg("-ICMSIS-DSP/Include")
        .clang_arg("-ICMSIS_5/CMSIS/Core/Include")
        // Presume the compiler works
        // This is necessary because the compiler checks rely on
        // having a standard library present
        .clang_arg("-DCMAKE_C_COMPILER_WORKS=1")
        // Don't use the rust standard library
        .use_core()
        // Invalidate the built crate if any of the included header files change
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    if is_crosscompile() {
        builder = builder
            .clang_arg("-Ipicolibc/newlib/libc/include")
            .clang_arg(format!(
                "-I{}/build/picolibc/picolibc/include",
                build_path.display()
            ))
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build() -> PathBuf {
    let mut config = cmake::Config::new(".");
    config.build_target("CMSISDSP");
    if is_crosscompile() {
        config.define("CMAKE_C_COMPILER_WORKS", "1");
        config.define("CMAKE_CXX_COMPILER_WORKS", "1");
    }
    let dst = config.build();
    println!(
        "cargo:rustc-link-search=native={}/build/CMSIS-DSP/Source/",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=CMSISDSP");
    dst
}
