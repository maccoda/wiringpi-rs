extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-search=native=/usr/local/include/wiringPi/wiringPi");
    println!("cargo:rustc-link-lib=wiringPi");
    // Crpyt needs to be linked last (need to look into why this is)
    println!("cargo:rustc-link-lib=crypt");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        // .header("../WiringPi/wiringPi/wiringPi.h")
        // .header("../WiringPi/wiringPi/wiringSerial.h")
        .header("wrapper.h")
    // #[cfg(feature = "serial")]
    // let bindings: bindgen::Builder = bindings.header("../WiringPi/wiringPi/wiringSerial.h");


    // let bindings = bindings
        // Want some comments
        .generate_comments(true)
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
