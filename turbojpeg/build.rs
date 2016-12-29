fn main() {
    println!("cargo:rustc-link-search=/usr/local/opt/jpeg-turbo/lib");
    println!("cargo:rustc-link-lib=turbojpeg");
}
