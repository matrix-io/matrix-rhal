use std::{env, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!(
        "cargo:rustc-link-search=native={}",
        root.join("build/console/").display()
    );
    // Link to matrixio_hal_esp32 generated `hal` library
    println!("cargo:rustc-link-lib=static=console");
}
