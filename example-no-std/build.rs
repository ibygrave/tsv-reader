use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    Command::new(manifest_path.join("generate.py"))
        .args([out_path.join("data.tsv")])
        .spawn()
        .unwrap();
}
