mod stubs {
    include!(concat!(env!("OUT_DIR"), "/stubs.rs"));
}

fn main() {
    println!("Starting");

    let path = utils::find_deps_dylib("exporter").unwrap();
    println!("Loading {}", path.display());
    stubs::exporter_stub.load_from(&path).unwrap();
    stubs::base.resolve_uncached().unwrap();

    let result = importer::addition(0);
    println!("result: {}", result);

    assert!(!stubs::missing.resolve());

    println!("OK");
}

#[test]
fn test_main() {
    main();
}
