fn main() {
    // MultitouchSupport is a private framework located in PrivateFrameworks
    println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
}

