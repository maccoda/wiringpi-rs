extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // Build the wiringPi library
    match Command::new("make").arg("-e").arg("wiringpi").status() {
        Ok(status) => {
            if !status.success() {
                panic!("failed to build wiringPi C library (exit code {:?})",
                       status.code())
            }
        }
        Err(e) => panic!("failed to build wiringPi C library: {}", e),
    }
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=wiringpi");
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
    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
