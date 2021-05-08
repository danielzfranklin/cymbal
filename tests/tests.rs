use std::{fs, path::PathBuf};

use duct::cmd;
use insta::assert_debug_snapshot;
use memmap::Mmap;

use cymbal::ParsedDwarf;

fn build_sample(name: &str) -> eyre::Result<()> {
    let ws = PathBuf::from("tests/samples").join(name);

    let target = PathBuf::from("tests/samples/.targets").join(name);
    fs::create_dir_all(&target)?;

    cmd!("cargo", "build", "--target-dir", fs::canonicalize(target)?)
        .stdout_null()
        .stderr_null()
        .dir(ws)
        .run()?;

    Ok(())
}

fn hello_world_bin() -> eyre::Result<Mmap> {
    build_sample("hello_world")?;
    let bin = PathBuf::from("tests/samples/.targets/hello_world/debug/hello_world");
    let bin = fs::File::open(bin)?;
    // Safety: We don't modify this file, and we assume no one else modifies
    //   files in our sample directory
    let data = unsafe { Mmap::map(&bin) }?;
    Ok(data)
}

#[test]
fn parse_dwarf_hello_world() -> eyre::Result<()> {
    let data = hello_world_bin()?;
    assert_debug_snapshot!(ParsedDwarf::new(&*data).unwrap());
    Ok(())
}
