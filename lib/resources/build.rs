use std::process::Command;

fn main() {
    let git_hash = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(||String::from("<Not built in git repository>"));

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
