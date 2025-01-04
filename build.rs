use std::process::Command;

const CC: &str = "arm-none-eabi-as";
const ARGS: &[&str] = &["-mcpu=cortex-m4", "-mthumb", "-Wall"];

fn main() {
    println!("cargo:rerun-if-changed=boot.S");
    println!("Running build.rs...");

    let output = Command::new(CC)
        .args(ARGS)
        .arg("boot.S")
        .arg("-o")
        .arg("target/thumbv7em-none-eabi/debug/boot.o")
        .status()
        .expect("Failed to assemble boot.S");

    if !output.success() {
        println!("Assembly of boot.S failed");
    }
}
