use std::env;
use std::path::PathBuf;

fn main() {
    bindgen();
    build();
}

fn bindgen() {
    // Invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Only generate bindings for the arm_ functions and variables
        .allowlist_function("arm_.*")
        .allowlist_var("arm_.*")
        // Where to look for headers
        .clang_arg("-ICMSIS-DSP/Include")
        .clang_arg("-ICMSIS_5/CMSIS/Core/Include")
        // Don't use the rust standard library
        .use_core()
        // Invalidate the built crate if any of the included header files change
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build() {
    let dst = cmake::Config::new(".").build_target("CMSISDSP").build();
    println!(
        "cargo:rustc-link-search=native={}/build/CMSIS-DSP/Source/",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=CMSISDSP");
}
