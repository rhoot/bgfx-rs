use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_quad = target.split("-").collect::<Vec<_>>();
    let (arch, _, os, compiler) = (target_quad[0], target_quad[1], target_quad[2], target_quad[3]);
    let bitness = if arch == "x86_64" { 64 } else { 32 };

    match os {
        "windows" => build_windows(bitness, compiler),
        "linux"   => build_linux(bitness, compiler),
        _         => panic!("Unsupported platform"),
    }
}

fn build_windows(bitness: u32, compiler: &str) {
    if compiler != "gnu" {
        panic!("Unsupported compiler");
    }

    Command::new("make.exe")
        .current_dir("bgfx")
        .arg(".build/projects/gmake-mingw-gcc")
        .output()
        .unwrap();

    Command::new("make.exe")
        .current_dir("bgfx")
        .arg("-R")
        .arg("-C")
        .arg(".build/projects/gmake-mingw-gcc")
        .arg(format!("config=release{}", bitness))
        .arg("bgfx")
        .output()
        .unwrap();

    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(format!("win{}_mingw-gcc", bitness));
    path.push("bin");

    println!("cargo:rustc-link-lib=static=bgfxRelease");
    println!("cargo:rustc-link-search={}", path.as_os_str().to_str().unwrap());
}

fn build_linux(bitness: u32, compiler: &str) {
    if compiler != "gnu" {
        panic!("Unsupported compiler");
    }

    // Generate makefiles
    Command::new("make")
        .current_dir("bgfx")
        .arg(".build/projects/gmake-linux")
        .output()
        .unwrap();

    // Build bgfx
    let status = Command::new("make")
        .current_dir("bgfx")
        .env("CPPFLAGS", "-fPIC")
        .env("CFLAGS", "-fPIC")
        .arg("-R")
        .arg("-C")
        .arg(".build/projects/gmake-linux")
        .arg(format!("config=release{}", bitness))
        .arg("bgfx")
        .status()
        .unwrap_or_else(|e| panic!("Failed to build bgfx: {}", e));

    if status.code().unwrap() != 0 {
        panic!("Failed to build bgfx.");
    }

    // Output linker config
    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(format!("linux{}_gcc", bitness));
    path.push("bin");

    println!("cargo:rustc-link-lib=bgfxRelease");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-search=native={}", path.as_os_str().to_str().unwrap());
}