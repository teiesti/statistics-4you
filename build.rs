use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let head = String::from_utf8_lossy(&output.stdout);
    println!("cargo:rustc-env=PKG_COMMIT={}", head.trim());
}
