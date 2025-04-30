mod stubs {
    include!(concat!(env!("OUT_DIR"), "/stubs.rs"));
}

fn main() {
    println!("Starting");

    let path = utils::find_deps_dylib("exporter").unwrap();
    println!("Loading {}", path.display());
    stubs::exporter_stub.load_from(&path).unwrap();

    // Test scoped resolution
    let token = stubs::base.resolve().unwrap();
    let result = importer::addition1(0);
    println!("result 1: {}", result);
    drop(token);

    #[cfg(feature = "checked")]
    assert_eq!(unsafe { importer::get_SOMEDATA() }, std::ptr::null());

    // Test resolve_global()
    stubs::base.resolve().unwrap().mark_permanent();
    let result = importer::addition2(0);
    println!("result 2: {}", result);

    // Test resolution of missing symbols
    assert!(stubs::missing.resolve().is_err());

    println!("OK");
}

#[test]
fn test_main() {
    main();
}
