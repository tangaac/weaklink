use std::env;
use std::path::Path;

use weaklink_build::exports::dylib_exports;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args = Vec::from_iter(env::args());
    let exports = dylib_exports(Path::new(&args[1]))?;
    for exp in &exports {
        println!("{} {:?}", exp.name, exp.section);
    }
    Ok(())
}

#[test]
fn test_exports() {
    let path = utils::find_deps_dylib("exporter").unwrap();
    let exports = dylib_exports(&path).unwrap();

    let mut adds_found = 0;
    for exp in exports {
        println!("name={}, section={:?}", exp.name, exp.section);
        let exp_name = exp.name.trim_start_matches('_').trim_end_matches(|c: char| c.is_numeric());
        if exp_name == "add_" {
            adds_found += 1;
        }
    }
    assert_eq!(adds_found, 10);
}
