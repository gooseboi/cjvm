#![deny(
    clippy::enum_glob_use,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used
)]

mod class_file;
mod jar;
mod utils;
use class_file::{
    ClassFile, ConstantInfo, FieldDescriptor, MethodAccessFlags, MethodDescriptor, MethodInfo,
};

use camino::Utf8PathBuf;
use clap::Parser as _;
use rc_zip_sync::ReadZip as _;
use std::fs::File;

#[derive(clap::Parser)]
struct Cmd {
    #[arg()]
    fname: Utf8PathBuf,
}

fn is_main_method(main_class: &ClassFile, method: &MethodInfo) -> bool {
    let ConstantInfo::Utf8 { bytes } = &main_class.constant_pool[method.name_index as usize] else {
        panic!("method name_index was not Utf8");
    };

    // It's called main
    if bytes != "main" {
        return false;
    }

    // It's public and static
    if method.access_flags != (MethodAccessFlags::Public | MethodAccessFlags::Static) {
        return false;
    }

    let ConstantInfo::Utf8 { bytes: descriptor } =
        &main_class.constant_pool[method.descriptor_index as usize]
    else {
        panic!("method descriptor was not Utf8");
    };

    let descriptor = MethodDescriptor::parse(descriptor);

    // It should return void
    let FieldDescriptor::Void = descriptor.return_ty else {
        return false;
    };

    // It can only have one
    let [param] = &descriptor.parameters[..] else {
        return false;
    };

    // It should take an array
    let FieldDescriptor::Array { ty } = param else {
        return false;
    };

    // The array should hold a class
    let FieldDescriptor::Class { class_name } = ty.as_ref() else {
        return false;
    };

    // The class in the array should be a String
    if class_name != "java/lang/String" {
        return false;
    }

    true
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

    let main_class = ClassFile::parse(&main_class);
    println!("finished parsing the main class file");
    println!();

    if let Some(main_method) = main_class
        .methods
        .iter()
        .find(|m| is_main_method(&main_class, m))
    {
        println!("found main_method: {main_method:?}");
    } else {
        println!("there was no main method D:");
    }
}
