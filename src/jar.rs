// Jar file spec: https://docs.oracle.com/en/java/javase/20/docs/specs/jar/jar.html

use std::collections::HashMap;

pub struct Manifest {
    pub version: String,
    pub main_class_name: String,
}

pub fn parse_manifest(manifest: &str) -> Manifest {
    let mut manifest: HashMap<String, String> = manifest
        .lines()
        .filter_map(|s| s.split_once(':'))
        .map(|(s1, s2)| (s1.into(), s2.trim().into()))
        .collect();

    let version = manifest
        .remove("Manifest-Version")
        .expect("manifest should have a version");
    let main_class_name = manifest
        .remove("Main-Class")
        .expect("Jar can't be compiled down without main class");

    Manifest {
        version,
        main_class_name,
    }
}
