// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

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
}

#[cfg(target_os = "windows")]
fn build_bgfx_msvc(bitness: u32) {
    let vs_version = env::var("VisualStudioVersion").expect("Visual Studio version not detected");
    let platform = if bitness == 32 { "X86" } else { "X64" };

    let vs_release = match vs_version.as_ref() {
        "12.0" => "2013",
        "14.0" => "2015",
        _ => panic!(format!("Unknown Visual Studio version: {:?}", vs_version)),
    };

    Command::new("make.exe")
        .current_dir("bgfx")
        .arg(format!(".build/projects/vs{}", vs_release))
        .output()
        .unwrap();

    let status = Command::new("MSBuild.exe")
                     .current_dir("bgfx")
                     .arg("/p:Configuration=Release")
                     .arg(format!("/p:Platform={}", platform))
                     .arg(format!(".build/projects/vs{}/bgfx.vcxproj", vs_release))
                     .status()
                     .expect("Failed to build bgfx");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bgfx");
    }

    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(format!("win{}_vs{}", bitness, vs_release));
    path.push("bin");

    println!("cargo:rustc-link-lib=static=bgfxRelease");
    println!("cargo:rustc-link-search=native={}", path.as_os_str().to_str().unwrap());
}

#[cfg(target_os = "windows")]
fn build_bgfx(bitness: u32, compiler: &str, profile: &str) {
    if compiler == "msvc" {
        build_bgfx_msvc(bitness);
        return;
    }

    Command::new("make.exe")
        .current_dir("bgfx")
        .arg(".build/projects/gmake-mingw-gcc")
        .output()
        .unwrap();

    let status = Command::new("make.exe")
                     .current_dir("bgfx")
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
