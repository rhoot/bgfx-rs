extern crate gcc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let target_quad = target.split("-").collect::<Vec<_>>();
    let (arch, compiler) = (target_quad.first().unwrap(), target_quad.last().unwrap());
    let bitness = if *arch == "x86_64" { 64 } else { 32 };

    build_bgfx(bitness, compiler, &profile);
    build_bx_helper();
}

fn build_bx_helper() {
    gcc::Config::new()
        .cpp(true)
        .include("bx/include")
        .file("src/bx-helper.cpp")
        .compile("libbxhelper.a");
}

#[cfg(target_os = "windows")]
fn build_bgfx(bitness: u32, compiler: &str, profile: &str) {
    if compiler != "gnu" {
        panic!("Unsupported compiler");
    }

    Command::new("make.exe")
        .current_dir("bgfx")
        .arg(".build/projects/gmake-mingw-gcc")
        .output()
        .unwrap();

    let status = Command::new("make.exe")
                     .current_dir("bgfx")
                     .env("CPPFLAGS", "-fPIC -DBGFX_CONFIG_MULTITHREADED=1")
                     .env("CFLAGS", "-fPIC -DBGFX_CONFIG_MULTITHREADED=1")
                     .arg("-R")
                     .arg("-C")
                     .arg(".build/projects/gmake-mingw-gcc")
                     .arg(format!("config={}{}", profile, bitness))
                     .arg("bgfx")
                     .status()
                     .unwrap_or_else(|e| panic!("Failed to build bgfx: {}", e));

    if status.code().unwrap() != 0 {
        panic!("Failed to build bgfx.");
    }

    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(format!("win{}_mingw-gcc", bitness));
    path.push("bin");

    println!("cargo:rustc-link-lib=bgfx{}",
             if profile == "debug" { "Debug" } else { "Release" });
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=opengl32");
    println!("cargo:rustc-link-lib=psapi");
    println!("cargo:rustc-link-search=native={}", path.as_os_str().to_str().unwrap());
}

#[cfg(target_os = "linux")]
fn build_bgfx(bitness: u32, compiler: &str, profile: &str) {
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
                     .env("CPPFLAGS", "-fPIC -DBGFX_CONFIG_MULTITHREADED=1")
                     .env("CFLAGS", "-fPIC -DBGFX_CONFIG_MULTITHREADED=1")
                     .arg("-R")
                     .arg("-C")
                     .arg(".build/projects/gmake-linux")
                     .arg(format!("config={}{}", profile, bitness))
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

    println!("cargo:rustc-link-lib=bgfx{}",
             if profile == "debug" { "Debug" } else { "Release" });
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-search=native={}", path.as_os_str().to_str().unwrap());
}
