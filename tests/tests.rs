use std::{fs, path::PathBuf};

use duct::cmd;
use insta::assert_debug_snapshot;
use memmap::Mmap;

use cymbal::{Function, ParsedDwarf};

fn build_sample(ws_name: &str) -> eyre::Result<()> {
    let ws = PathBuf::from("tests/samples").join(ws_name);

    let target = PathBuf::from("tests/samples/.targets").join(ws_name);
    fs::create_dir_all(&target)?;

    let run = cmd!("cargo", "build", "--target-dir", fs::canonicalize(target)?)
        .stderr_capture()
        .stderr_null()
        .dir(ws)
        .unchecked()
        .run()?;

    if !run.status.success() {
        let stderr = String::from_utf8_lossy(&run.stderr);
        panic!("`cargo build` failed:\n{}", stderr);
    }

    Ok(())
}

fn bin(ws_name: &str, bin_name: &str) -> eyre::Result<Mmap> {
    build_sample(ws_name)?;
    let bin = PathBuf::from(format!(
        "tests/samples/.targets/{}/debug/{}",
        ws_name, bin_name
    ));
    let bin = fs::File::open(bin)?;
    // Safety: We don't modify this file, and we assume no one else modifies
    //   files in our sample directory
    let data = unsafe { Mmap::map(&bin) }?;
    Ok(data)
}

fn hello_world_bin() -> eyre::Result<Mmap> {
    bin("hello_world", "hello_world")
}

fn simple_bin() -> eyre::Result<Mmap> {
    bin("simple", "blog")
}

fn functions<'a>(dwarf: &'a ParsedDwarf) -> Vec<&'a Function> {
    let mut functions = dwarf.functions().iter().collect::<Vec<_>>();
    functions.sort();
    functions
}

fn vars<'a>(dwarf: &'a ParsedDwarf) -> Vec<&'a str> {
    let mut vars = dwarf.var_names().collect::<Vec<_>>();
    vars.sort_unstable();
    vars
}

fn symbols<'a>(dwarf: &'a ParsedDwarf) -> Vec<&'a str> {
    let mut symbols = dwarf
        .symbols()
        .iter()
        .filter_map(|s| s.demangled_name())
        .collect::<Vec<_>>();
    symbols.sort_unstable();
    symbols
}

#[test]
fn parse_dwarf_hello_world() -> eyre::Result<()> {
    let data = hello_world_bin()?;
    let dwarf = ParsedDwarf::new(&*data)?;
    assert_debug_snapshot!("hello_world_symbols", symbols(&dwarf));
    assert_debug_snapshot!("hello_world_vars", vars(&dwarf));
    assert_debug_snapshot!("hello_world_functions", functions(&dwarf));
    Ok(())
}

#[test]
fn parse_dwarf_simple() -> eyre::Result<()> {
    let data = simple_bin()?;
    let dwarf = ParsedDwarf::new(&*data)?;
    assert_debug_snapshot!("simple_symbols", symbols(&dwarf));
    assert_debug_snapshot!("simple_vars", vars(&dwarf));
    assert_debug_snapshot!("simple_functions", functions(&dwarf));
    Ok(())
}
