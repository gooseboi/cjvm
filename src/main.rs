#![deny(
    clippy::enum_glob_use,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used
)]

// Useful links:
// Full spec: https://docs.oracle.com/en/java/javase/20/docs/specs/jar/jar.html

use camino::Utf8PathBuf;
use clap::Parser as _;
use rc_zip_sync::ReadZip;
use std::{collections::HashMap, fs::File};

#[derive(clap::Parser)]
struct Cmd {
    #[arg()]
    fname: Utf8PathBuf,
}

fn parse_manifest(manifest: &str) -> HashMap<String, String> {
    manifest
        .lines()
        .filter_map(|s| s.split_once(':'))
        .map(|(s1, s2)| (s1.into(), s2.trim().into()))
        .collect()
}

fn main() {
    let cmd = Cmd::parse();
    let file = File::options()
        .read(true)
        .write(false)
        .open(cmd.fname)
        .expect("failed opening jar file");

    let zip = file
        .read_zip()
        .expect("file should be a jar file (a zip file)");

    let entry = zip
        .by_name("META-INF/MANIFEST.MF")
        .expect("there should be a manifest file");

    let manifest = entry.bytes().expect("should be able to read out manifest");
    let manifest = str::from_utf8(&manifest).expect("manifest is UTF-8 text");
    let manifest = parse_manifest(manifest);
    println!("Parsed manifest");

    if manifest
        .get("Manifest-Version")
        .expect("manifest should have a version")
        != "1.0"
    {
        panic!("Only manifest version supported is 1.0");
    }

    let main_class_name = manifest
        .get("Main-Class")
        .expect("Jar can't be compiled down without main class");
    println!("Main class is {main_class_name}");

    let mut main_class_path = main_class_name.replace('.', "/");
    main_class_path.push_str(".class");
    let main_class = zip
        .by_name(main_class_path)
        .expect("main class should be in jar");
    let main_class = main_class
        .bytes()
        .expect("should be able to read out main_class");

    println!("main class is {} bytes long", main_class.len());

    println!();
    println!("All entries: ");
    for entry in zip.entries() {
        if entry.uncompressed_size == 0 {
            // It's a directory
            continue;
        }

        println!("{}:{}", entry.uncompressed_size, entry.name);
    }
}
