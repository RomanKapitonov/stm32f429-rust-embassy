use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let ext_dir = manifest_dir.join("ext");
    let build_dir = ext_dir.join("build");

    let status = Command::new("make")
        .arg("-C")
        .env(
            "CFLAGS",
            env::var("PROFILE")
                .map(|arg| match arg.as_str() {
                    "debug" => "-O0",
                    "release" => "-O3",
                    _ => panic!("unknown profile: {}", arg),
                })
                .inspect(|opt_flag| {
                    dbg!(opt_flag);
                })
                .unwrap_or("-O0"),
        )
        .arg(&ext_dir)
        .status()
        .expect("Failed to execute 'make'");

    if !status.success() {
        panic!("Makefile execution failed with status: {}", status);
    }

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=ws28xx");

    println!("cargo:rustc-link-arg=--lto-O3");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    println!("cargo:rerun-if-changed=ext/Makefile");
    println!("cargo:rerun-if-changed=ext/src");

    println!("cargo:rustc-link-arg=--gc-sections");

    // Remap interrupt symbols due to embassy-stm32 issues
    // see https://github.com/embassy-rs/embassy/issues/4597
    println!("cargo:rustc-link-arg=--defsym=DMA2_STREAM2=DMA2_STREAM2_OVERRIDE");

    // Will be necessary due to some issues with embassy-stm32 implementation
    // see https://github.com/embassy-rs/embassy/issues/4597
    // println!("cargo:rustc-link-arg=--allow-multiple-definition");
}
