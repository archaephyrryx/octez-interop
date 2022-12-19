use std::{env, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let ocaml_main_dir = "./lib";
    let dune_dir = "_build/default/lib";
    Command::new("opam")
        .args(&["exec", "--", "dune", "build", &format!("{}/main.exe.o", ocaml_main_dir)])
        .status()
        .expect("Dune failed");
    Command::new("rm")
        .args(&["-f", &format!("{}/libmain.a", out_dir)])
        .status()
        .expect("rm failed");
    Command::new("rm")
        .args(&["-f", &format!("{}/libmain.o", out_dir)])
        .status()
        .expect("rm failed");
    Command::new("cp")
        .args(&[
            &format!("{}/main.exe.o", dune_dir),
            &format!("{}/libmain.o", out_dir),
        ])
        .status()
        .expect("File copy failed");
    Command::new("ar")
        .args(&[
            "qs",
            &format!("{}/libmain.a", out_dir),
            &format!("{}/libmain.o", out_dir),
        ])
        .status()
        .expect("ar failed");

    println!("cargo:rerun-if-changed={}/main.ml", ocaml_main_dir);
    println!("cargo:rerun-if-changed={}/dune", ocaml_main_dir);
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=main");
}