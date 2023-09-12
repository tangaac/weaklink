mod stubs {
    include!(concat!(env!("OUT_DIR"), "/stubs.rs"));
}

fn main() {
    println!("Starting");

    let path = utils::find_deps_dylib("exporter").unwrap();
    println!("Loading {}", path.display());
    stubs::exporter_stub.load_from(&path).unwrap();

    // Test lazy binding
    let result = importer::addition1(0);
    println!("result 1: {}", result);

    // Test resolution of API group
    stubs::base.resolve_uncached().unwrap();
    let result = importer::addition2(0);
    println!("result 2: {}", result);

    // Test resolution of missing symbols
    assert!(!stubs::missing.resolve());

    println!("OK");
}

#[test]
fn test_main() {
    main();
}
