use std::env;
use std::fs;

/* conditional compilation
 * see https://doc.rust-lang.org/reference/conditional-compilation.html */

/* environment variables Cargo sets for build scripts
 * see https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts */

const HIDAPI_DLL: &str = "hidapi.dll";

fn configure_hidapi(path: &str) {
    let cwd = env::current_dir().unwrap();
    let lib_dir = cwd.join(path);
    let target_dir = cwd.join("../target").join(env::var("PROFILE").unwrap());
    let from = lib_dir.join(HIDAPI_DLL);
    let to = target_dir.join(HIDAPI_DLL);
    if let Err(_) = fs::copy(from, to) {
        panic!("unable to copy {}", HIDAPI_DLL);
    }
    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());
}

#[cfg(target_arch = "x86_64")]
fn main() {
    configure_hidapi("vendor/hidapi/x64");
}

#[cfg(target_arch = "x86")]
fn main() {
    configure_hidapi("vendor/hidapi/x86");
}
