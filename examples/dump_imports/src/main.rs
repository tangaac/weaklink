use std::env;
use std::path::Path;

use weaklink_build::imports::archive_imports;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args = Vec::from_iter(env::args());
    let imports = archive_imports(Path::new(&args[1]))?;
    for imp in &imports {
        println!("{}", imp.name);
    }
    Ok(())
}

#[test]
fn test_imports() {
    let path = utils::find_latest_deps_artifact(|name| name.contains("importer") && name.ends_with(".rlib")).unwrap();
    let imports = archive_imports(&path).unwrap();

    let mut adds_found = 0;
    for imp in imports {
        println!("name={}", imp.name);
        let imp_name = imp.name.trim_start_matches('_').trim_end_matches(|c: char| c.is_numeric());
        if imp_name == "add_" {
            adds_found += 1;
        }
    }
    assert_eq!(adds_found, utils::num_repetitions());
}
