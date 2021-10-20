use std::{collections::HashMap, io::Error, process::Command};

pub fn get_derivation_json(file: &str, is_expression: bool) -> Result<serde_json::Value, Error> {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub pname: String,
    pub version: String,
    pub meta: Meta,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub license: Option<Licenses>,
    pub description: Option<String>,
    pub homepage: Option<Homepages>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Homepages {
    Homepage(String),
    HomepageList(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Licenses {
    License(License),
    LicenseList(Vec<License>),
    NameOnly(String),
    NameOnlyList(Vec<String>),
    SpecialCase(Vec<Licenses>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct License {
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    #[serde(rename = "spdxId")]
    pub spdx_id: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Out {
    pub path: String,
    #[serde(rename = "hashAlgo")]
    pub hash_algo: Option<String>,
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Drv {
    pub outputs: HashMap<String, Out>,
    #[serde(rename = "inputSrcs")]
    pub input_srcs: Vec<String>,
    #[serde(rename = "inputDrvs")]
    pub input_drvs: HashMap<String, Vec<String>>,
    pub system: String,
    pub builder: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

pub trait Derivation {
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