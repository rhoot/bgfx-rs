// Copyright (c) 2015, Johan Sköld.
// License: http://opensource.org/licenses/ISC

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let first_div = target.find('-').unwrap();
    let last_div = target.rfind('-').unwrap();

    let arch = &target[..first_div];
    let platform = &target[(first_div + 1)..last_div];
    let compiler = &target[(last_div + 1)..];
    let bitness = if arch == "x86_64" { 64 } else { 32 };

    match compiler {
        "msvc" => build_msvc(bitness),
        "gnu" => build_gnu(bitness, &profile, platform),
        _ => unreachable!(),
    }
}

fn build_msvc(bitness: u32) {
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
        .expect("Failed to generate project files");

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

fn build_gnu(bitness: u32, profile: &str, platform: &str) {
    let project_name = match platform {
        "windows" => "gmake-mingw-gcc",
        "unknown-linux" => "gmake-linux",
        _ => unreachable!(),
    };

    let output_name = match platform {
        "windows" => format!("win{}_mingw-gcc", bitness),
        "unknown-linux" => format!("linux{}_gcc", bitness),
        _ => unreachable!(),
    };

    // Generate makefiles
    Command::new("make")
        .current_dir("bgfx")
        .arg(format!(".build/projects/{}", project_name))
        .output()
        .expect("Failed to generate makefiles");

    // Build bgfx
    let status = Command::new("make")
                     .current_dir("bgfx")
                     .env("CFLAGS", "-fPIC -DBGFX_CONFIG_MULTITHREADED=1")
                     .arg("-R")
                     .arg("-C")
                     .arg(format!(".build/projects/{}", project_name))
                     .arg(format!("config={}{}", profile, bitness))
                     .arg("bgfx")
                     .status()
                     .expect("Failed to build bgfx");

    if status.code().unwrap() != 0 {
        panic!("Failed to build bgfx.");
    }

    // Output linker config
    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("bgfx");
    path.push(".build");
    path.push(output_name);
    path.push("bin");

    let config = if profile == "debug" { "Debug" } else { "Release" };
    println!("cargo:rustc-link-lib=bgfx{}", config);
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-search=native={}", path.as_os_str().to_str().unwrap());

    match platform {
        "windows" => {
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=opengl32");
            println!("cargo:rustc-link-lib=psapi");
        }
        "unknown-linux" => {
            println!("cargo:rustc-link-lib=GL");
            println!("cargo:rustc-link-lib=X11");
        }
        _ => unreachable!(),
    }
}
