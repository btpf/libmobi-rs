use std::env;
use std::path::PathBuf;
use std::env::consts::OS;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // Set the path to the directory containing the C library
    let lib_dir = "./libs"; // Replace with the actual path to the directory

    // Link the C library at build time
    println!("cargo:rustc-link-search={}/{}", lib_dir, OS);

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=mobitoolmod"); // We have touchdown with this line of code. Now we just need it to work dynamically
    println!("cargo:rustc-link-lib=static=mobitoolmod");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./libs/mobitoolmod.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
