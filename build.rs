fn main() {
    // Only link to the Accelerate framework when building for macOS
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
}
