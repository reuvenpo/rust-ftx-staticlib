use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    std::fs::copy(
        format!("{}/../enclave/target/x86_64-fortanix-unknown-sgx/debug/libenclave_check.a", crate_dir),
        format!("{}/libenclave_check.a", crate_dir),
    ).unwrap();

    println!("cargo:rustc-link-search={}", crate_dir);
    println!("cargo:rustc-link-lib=enclave_check");
}
