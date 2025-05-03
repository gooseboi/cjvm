#![deny(
    clippy::enum_glob_use,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used
)]

mod class_file;
mod jar;
mod utils;

use camino::Utf8PathBuf;
use clap::Parser as _;
use rc_zip_sync::ReadZip as _;
use std::fs::File;

#[derive(clap::Parser)]
struct Cmd {
    #[arg()]
    fname: Utf8PathBuf,
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

    let manifest = zip
        .by_name("META-INF/MANIFEST.MF")
        .expect("there should be a manifest file")
        .bytes()
        .expect("should be able to read out manifest");
    let manifest = str::from_utf8(&manifest).expect("manifest is UTF-8 text");
    let manifest = jar::parse_manifest(manifest);
    println!("Parsed manifest");

    assert!(
        manifest.version == "1.0",
        "Only manifest version supported is 1.0"
    );

    println!("Main class is {}", manifest.main_class_name);

    let main_class_path = manifest.main_class_name;
    let mut main_class_path = main_class_path.replace('.', "/");
    main_class_path.push_str(".class");
    println!("Searching for {main_class_path} in jar");
    let main_class = zip
        .by_name(main_class_path)
        .expect("main class should be in jar")
        .bytes()
        .expect("should be able to read out main_class");

    println!("main class is {} bytes long", main_class.len());

    let main_class = class_file::parse_class_file(&main_class);
    println!("{main_class:?}");
}
