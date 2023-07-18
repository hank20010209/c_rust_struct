use std::env;
fn main() {
    let path = env::current_dir().unwrap();
    println!("cargo:rustc-link-search={}/src/clib", path.to_string_lossy());
    println!("cargo:rustc-link-lib=static=cool");
}