use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

pub fn add_runner(command: Command) -> Command {
    for (key, value) in env::vars() {
        if key.starts_with("CARGO_TARGET_") && key.ends_with("_RUNNER") {
            let mut parts = value.split(' ');
            let mut runner = Command::new(parts.next().unwrap());
            runner.args(parts);
            runner.arg(command.get_program());
            runner.args(command.get_args());
            return runner;
        }
    }
    command
}

pub fn find_latest_deps_artifact(pattern: impl Fn(&str) -> bool) -> Result<PathBuf, io::Error> {
    let mut deps_dir;
    if let Ok(out_dir) = env::var("OUT_DIR") {
        // e.g. target/debug/build/weak_linkage-2156dda05d199c7e/out
        deps_dir = PathBuf::from(out_dir);
        deps_dir.pop();
        deps_dir.pop();
        deps_dir.pop();
        deps_dir.push("deps");
    } else {
        deps_dir = PathBuf::from(env::current_exe().unwrap());
        deps_dir.pop();
        if !deps_dir.ends_with("deps") {
            deps_dir.push("deps");
        }
    }

    let mut entries: Vec<_> = deps_dir
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            let fname = e.file_name();
            pattern(fname.to_str().unwrap())
        })
        .collect();

    // Sort by modification time (latest first)
    entries.sort_by_key(|e| e.metadata().unwrap().modified().unwrap());
    match entries.last() {
        Some(e) => Ok(e.path()),
        None => Err(io::Error::from(io::ErrorKind::NotFound)),
    }
}

pub fn find_deps_dylib(name: &str) -> Result<PathBuf, io::Error> {
    let target = env::var("TARGET").unwrap();
    let filename = if target.contains("linux") {
        format!("lib{name}.so")
    } else if target.contains("darwin") {
        format!("lib{name}.dylib")
    } else if target.contains("windows") {
        format!("{name}.dll")
    } else {
        unreachable!()
    };
    find_latest_deps_artifact(|name| name == filename)
}
