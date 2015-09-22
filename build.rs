use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_quad = target.split("-").collect::<Vec<_>>();
    let (arch, _, os, compiler) = (target_quad[0], target_quad[1], target_quad[2], target_quad[3]);
    let bitness = if arch == "x86_64" { 64 } else { 32 };

    // Windows
    if os == "windows" {
        // Currently only supporting mingw
        if compiler != "gnu" {
            panic!("Unsupported compiler");
        }

        // projects
        {
            Command::new("make.exe")
                .current_dir("bgfx")
                .arg(".build/projects/gmake-mingw-gcc")
                .output()
                .unwrap();
        }

        // lib
        {
            Command::new("make.exe")
                .current_dir("bgfx")
                .arg("-R")
                .arg("-C")
                .arg(".build/projects/gmake-mingw-gcc")
                .arg(format!("config=release{}", bitness))
                .arg("bgfx")
                .output()
                .unwrap();
        }

        // link config
        {
            let mut path = PathBuf::from(env::current_dir().unwrap());
            path.push("bgfx");
            path.push(".build");
            path.push(format!("win{}_mingw-gcc", bitness));
            path.push("bin");

            println!("cargo:rustc-link-lib=static=bgfxRelease");
            println!("cargo:rustc-link-search={}", path.as_os_str().to_str().unwrap());
        }
    }
}
