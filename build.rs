use std::{env, path::PathBuf};

use git2::Repository;

const NAME: &str = "dungeonator";
const DEPS: &[&str] = &["pcg_basic"];

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let repo_path = out_path.join(NAME);
    if !repo_path.exists() {
        Repository::clone(
            &format!("https://github.com/BenjaminHinchliff/{}.git", NAME),
            &repo_path,
        )
        .unwrap();
    }
    cmake::build(&repo_path);

    println!("cargo:rerun-if-changed=wrapper.h");

    println!("cargo:rustc-link-lib={}", NAME);
    for dep in DEPS {
        println!("cargo:rustc-link-lib={}", dep);
    }
    println!(
        "cargo:rustc-link-search={}",
        out_path.join("lib").to_string_lossy()
    );

    let bindings = bindgen::Builder::default()
        .clang_arg(&format!("-I{}", out_path.join("include").to_string_lossy()))
        .header("wrapper.h")
        .whitelist_type("Grid")
        .whitelist_function("seedDungeonatorRNG")
        .whitelist_function("generateDungeon")
        .whitelist_function("freeGrid")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate dungeonator bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write out bindings");
}
