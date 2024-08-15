use serde::Deserialize;
use std::fs;

fn main() {
    // Read holochain version from the Cargo.toml file
    let cargo_toml_content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let cargo_toml: CargoToml =
        toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");
    let holochain_version = &cargo_toml.dependencies.holochain.version;
    println!("cargo:rustc-env=HOLOCHAIN_VERSION={}", holochain_version);

    tauri_build::build()
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    dependencies: Dependencies,
}

#[derive(Debug, Deserialize)]
struct Dependencies {
    holochain: HolochainDependency,
}

#[derive(Debug, Deserialize)]
struct HolochainDependency {
    version: String,
}
