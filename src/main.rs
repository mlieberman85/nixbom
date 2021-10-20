mod spdx_spec;
mod protos;
mod sbom;
mod nix;
use protobuf::text_format::print_to_string;


extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::prelude::*;
use clap::{App, Arg};
use nix::Package;
use protobuf::RepeatedField;
use spdx_spec::CreationInfo;
use spdx_spec::Document;
use spdx_spec::Package as SPDXPackage;
use spdx_spec::SpdxSchema;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::process::Command;
use uuid::Uuid;
use nix::{Drv, Derivation, get_derivation_json};

fn get_packages_wrapper(with_cache: bool) -> Result<HashMap<String, Package>, Error> {
    if with_cache {
        Ok(package_fixer(get_packages_cached("nixpkgs.json")?)) // TODO: This shouldn't be hardcoded and be configurable
    } else {
        Ok(package_fixer(get_packages()?))
    }
}

fn package_fixer(packages: HashMap<String, Package>) -> HashMap<String, Package> {
    packages
    .into_iter()
    .map(|(_, v)| (v.name.clone(), v))
    .collect::<HashMap<String, Package>>()
}

fn get_packages() -> serde_json::Result<HashMap<String, Package>> {
    let output = Command::new("nix-env")
        .arg("-qa")
        .arg(".*")
        .arg("--json")
        .arg("--meta")
        .output()
        .unwrap();

    // TODO: Below shouldn't live in here and should be cleaned up. Should be configurable and not hard coded.
    match File::create("nixpkgs.json")
            .and_then(|mut f| f.write_all(&output.stdout)) {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e),
    }

    serde_json::from_slice(&output.stdout)
}

fn get_packages_cached(path: &str) -> Result<HashMap<String, Package>, Error> {
    let path = Path::new(&path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let packages: HashMap<String, Package> = serde_json::from_reader(reader)?;
    let mapped = packages
        .into_iter()
        .map(|(_, v)| (v.name.clone(), v))
        .collect::<HashMap<String, Package>>();

    Ok(mapped)
}



fn main() -> Result<(), Error> {
    let matches = App::new("Nixbom")
        .version("0.1")
        .author("Michael Lieberman and Jack Kelly")
        .arg(
            Arg::with_name("DERIVATION")
                .help("Sets the derivation to generate a SPDX Software Bill of Materials for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("name")
                .help("Name of the project you are building SBOM for.")
                .short("n")
                .long("name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("authors")
                .help("Author(s) of the project")
                .short("a")
                .long("authors")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("data_license")
                .help("Data License for the SBOM")
                .short("l")
                .long("license")
                .takes_value(true)
                .default_value("CC0-1.0"), // TODO: Make part of configuration?
        )
        .arg(
            Arg::with_name("with_cache")
                .help("Whether or not to use a Nix package cache json.")
                .long("--with_cache")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    // TODO: Handle failure better
    let derivation = matches.value_of("DERIVATION").unwrap();
    let json = get_derivation_json(derivation, true)?;
    let derivation_struct_map: HashMap<String, Drv> = serde_json::from_value(json)?;
    let input_derivations = derivation_struct_map.get_input_derivations();

    let packages = get_packages_wrapper(matches.is_present("with_cache"))?;
    let sbom = SpdxSchema::new(
        matches.value_of("name").unwrap().to_string(),
        Utc::now().to_string(),
        matches
            .values_of("authors")
            .unwrap()
            .map(|v| v.to_string())
            .collect(), // TODO: Maybe even pull info from git config if exists?
        matches.value_of("data_license").unwrap().to_string(),
        input_derivations
            .into_iter()
            .map(|v| v.env["name"].clone())
            .collect(),
        packages,
    );

    /*let mut cyclonedx_sbom = bom_1_3::Bom::new();
    cyclonedx_sbom.set_spec_version("1.3".to_string()); // TODO: Make this parameterizable
    cyclonedx_sbom.set_serial_number(Uuid::new_v4().to_string());
    cyclonedx_sbom.set_compositions(RepeatedField::from_vec(vec![]));*/

    /*cyclonedxdx_sbom.set_metadata(bom_1_3::Metadata {
        tools: (),
        authors: (),
        properties: (),
        _timestamp: (),
        _component: (),
        _manufacture: (),
        _supplier: (),
        _licenses: (),
        unknown_fields: (),
        cached_size: (),
    })*/
/* 
    let mut cyclonedx_sbom = bom_1_3::Bom {
        spec_version: "1.3", // TODO: Parameterize this?
        components: (),
        services: (),
        external_references: (),
        dependencies: (),
        compositions: (),
        _version: 1,
        _serial_number: (),
        _metadata: (),
        unknown_fields: (),
        cached_size: (),
    }*/

    //println!("{}", serde_json::to_string(&cyclonedx_sbom).unwrap());
    println!("{}", serde_json::to_string_pretty(&sbom)?);

    Ok(())
}
