use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use weaklink_build::{exports, imports};
use weaklink_build::{Config, SymbolStub};

fn main() {
    println!("cargo:rustc-env=TARGET={}", std::env::var("TARGET").unwrap());

    let path = utils::find_deps_dylib("exporter").unwrap();
    let exports = exports::dylib_exports(&path).unwrap();

    let path = utils::find_latest_deps_artifact(|name| name.contains("importer") && name.ends_with(".rlib")).unwrap();
    let imports = imports::archive_imports(&path).unwrap();

    let imports_str = HashSet::<String>::from_iter(imports.iter().map(|i| i.name.clone()));
    let exports_str = HashSet::<String>::from_iter(exports.iter().map(|e| e.name.clone()));
    let common = exports_str.intersection(&imports_str).collect::<HashSet<_>>();
    let symbols = exports
        .into_iter()
        .filter(|e| common.contains(&e.name))
        .map(|e| SymbolStub::new(&e.name))
        .collect::<Vec<_>>();

    println!("cargo:warning=Found {} common symbols", symbols.len());

    let mut config = Config::new("exporter_stub");
    config.add_symbol_group("base", symbols).unwrap();

    let missing = vec![SymbolStub::new("foo"), SymbolStub::new_data("get_bar", "bar")];
    config.add_symbol_group("missing", missing).unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let source_path = out_dir.join("stubs.rs");
    let mut source = File::create(&source_path).unwrap();
    config.lazy_binding = true;
    config.generate_source(&mut source);
    println!("cargo:rerun-if-changed={}", source_path.display());
    println!("cargo:warning=Generated {}", source_path.display());
}
