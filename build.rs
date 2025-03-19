use rustc_version::Version;

fn main() {
    if rustc_version::version().unwrap() >= Version::parse("1.78.0").unwrap() {
        println!("cargo:rustc-cfg=rustc_is_at_least_1_78");
    }
}
