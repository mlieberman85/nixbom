mod spdx_spec;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::prelude::*;
use clap::{App, Arg};
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

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    pname: String,
    version: String,
    meta: Meta,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    license: Option<Licenses>,
    description: Option<String>,
    homepage: Option<Homepages>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Homepages {
    Homepage(String),
    HomepageList(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Licenses {
    License(License),
    LicenseList(Vec<License>),
    NameOnly(String),
    NameOnlyList(Vec<String>),
    SpecialCase(Vec<Licenses>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct License {
    #[serde(rename = "fullName")]
    full_name: Option<String>,
    #[serde(rename = "shortName")]
    short_name: Option<String>,
    #[serde(rename = "spdxId")]
    spdx_id: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Out {
    path: String,
    #[serde(rename = "hashAlgo")]
    hash_algo: Option<String>,
    hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Drv {
    outputs: HashMap<String, Out>,
    #[serde(rename = "inputSrcs")]
    input_srcs: Vec<String>,
    #[serde(rename = "inputDrvs")]
    input_drvs: HashMap<String, Vec<String>>,
    system: String,
    builder: String,
    args: Vec<String>,
    env: HashMap<String, String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

trait Derivation {
    fn get_inner_drv(&self) -> Vec<Drv>;
    fn get_input_derivations(&self) -> Vec<Drv>;
}

impl Derivation for HashMap<String, Drv> {
    fn get_inner_drv(&self) -> Vec<Drv> {
        self.into_iter().map(|(_, v)| v.clone()).collect()
    }

    fn get_input_derivations(&self) -> Vec<Drv> {
        self.get_inner_drv()
            .into_iter()
            .flat_map(|v| v.input_drvs.clone())
            .map(|(k, _)| k)
            .collect::<Vec<String>>()
            .into_iter()
            .flat_map(|v| serde_json::from_value(get_derivation_json(&v, false).unwrap()))
            .collect::<Vec<HashMap<String, Drv>>>()
            .into_iter()
            .flat_map(|v| v.get_inner_drv())
            .collect::<Vec<Drv>>()
    }
}

fn get_derivation_json(file: &str, is_expression: bool) -> Result<serde_json::Value, Error> {
    let output_bytes = get_derivation_bytes(file, is_expression)?;
    let json = bytes_to_json(output_bytes)?;
    Ok(json)
}

fn get_derivation_bytes(file: &str, is_expression: bool) -> Result<Vec<u8>, Error> {
    let output = if is_expression {
        Command::new("nix")
            .arg("show-derivation")
            .arg("-f")
            .arg(file)
            .output()
    } else {
        Command::new("nix")
            .arg("show-derivation")
            .arg(file)
            .output()
    }?;

    Ok(output.stdout)
}

fn bytes_to_json(bytes: Vec<u8>) -> serde_json::Result<serde_json::Value> {
    let v = serde_json::from_slice(&bytes)?;

    Ok(v)
}

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

trait SpdxPackages {
    fn get_spdx_package_info_if_exists(&self, package: String) -> Option<SPDXPackage>;
}

fn license_helper(license: License) -> Option<String> {
    license
        .spdx_id
        .or(license.full_name.or(license.short_name.or(license.url)))
}

impl SpdxPackages for HashMap<String, Package> {
    fn get_spdx_package_info_if_exists(&self, package_name: String) -> Option<SPDXPackage> {
        let package = self.get(&package_name)?;
        let license = package
            .meta
            .license
            .to_owned()
            .map(|v| match v {
                Licenses::License(l) => vec![license_helper(l)],
                Licenses::LicenseList(l) => l.into_iter().map(|v| license_helper(v)).collect(),
                Licenses::NameOnly(l) => vec![Some(l)],
                Licenses::NameOnlyList(l) => l.into_iter().map(|v| Some(v)).collect(),
                Licenses::SpecialCase(_) => vec![None], // TODO: Figure out what todo with special cases!
            })
            .into_iter()
            .flatten()
            .collect();

        let homepage = {
            package.meta.homepage.to_owned().map(|v| match v {
                Homepages::Homepage(h) => h,
                // Assume if a list of URLs pick the first one
                Homepages::HomepageList(h) => h[0].clone(),
            })
        };
        let s = SPDXPackage {
            annotations: None,
            attribution_texts: None,
            checksums: None,
            comment: None,
            copyright_text: None,
            description: package.meta.description.to_owned(),
            download_location: None,
            external_refs: None,
            files_analyzed: None,
            has_files: None,
            homepage: homepage,
            license_comments: None,
            license_info_from_files: license,
            name: None,
            originator: None,
            package_file_name: None,
            package_verification_code: None,
            source_info: None,
            summary: None,
            supplier: None,
            version_info: Some(package.version.to_owned()),
        };

        Some(s)
    }
}

impl SpdxSchema {
    fn new(
        name: String,
        created: String,
        creators: Vec<String>,
        data_license: String,
        materials: Vec<String>,
        package_data: HashMap<String, Package>,
    ) -> SpdxSchema {
        let creation_info = CreationInfo {
            comment: None,
            created: Some(created),
            creators: Some(creators),
            license_list_version: None,
        };

        let packages = materials
            .into_iter()
            .flat_map(|v| package_data.get_spdx_package_info_if_exists(v))
            .collect::<Vec<spdx_spec::Package>>();

        let document = Document {
            annotations: None,
            comment: None,
            creation_info: Some(creation_info),
            data_license: Some(data_license),
            describes_packages: None,
            external_document_refs: None,
            files: None, // TODO: Support file based SBOMs
            has_extracted_licensing_infos: None,
            name: Some(name),
            packages: Some(packages),
            relationships: None,
            revieweds: None,
            snippets: None,
            spdx_version: Some("SPDX-2.2".to_string()), // TODO: Support multiple SPDX versions
        };

        SpdxSchema {
            document: Some(document),
        }
    }
}

fn main() -> Result<(), Error> {
    let matches = App::new("nixbom")
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

    println!("{}", serde_json::to_string_pretty(&sbom)?);

    Ok(())
}
