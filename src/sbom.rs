
use std::collections::HashMap;

use crate::spdx_spec::CreationInfo;
use crate::spdx_spec::Document;
use crate::spdx_spec::Package as SPDXPackage;
use crate::spdx_spec::SpdxSchema;
use crate::spdx_spec;
use crate::protos::bom_1_3::Bom as CycloneDXSbom;

use crate::nix::{Licenses, License, Package, Homepages};

pub struct CommonSBOMInputs {
    sbom_type: SBOMType,
    name: String,
    timestamp: String,
    authors: Vec<String>,
    data_license: String
}

pub struct CommonPackageInfo {
    
}

pub fn generate_sbom(common_sbom_inputs: CommonSBOMInputs, materials: Vec<String>, packages_data: HashMap<String, Package>) -> SBOM {
    match common_sbom_inputs.sbom_type {
        SBOMType::Spdx => SBOM::Spdx(generate_spdx_sbom(common_sbom_inputs)),
        SBOMType::CycloneDX => SBOM::CycloneDX(generate_cyclonedx_sbom(common_sbom_inputs)),
    }
}

fn generate_spdx_sbom(common_sbom_inputs: CommonSBOMInputs) -> SpdxSchema {
    let sbom = SpdxSchema::new(
        common_sbom_inputs.name,
        common_sbom_inputs.timestamp,
        common_sbom_inputs.authors,
        common_sbom_inputs.data_license,
        materials,
        packages,
    );

    sbom
}

fn generate_cyclonedx_sbom(common_sbom_inputs: CommonSBOMInputs) -> CycloneDXSbom {

}

enum SBOMType {
    Spdx,
    CycloneDX
}

enum SBOM {
    Spdx(SpdxSchema),
    CycloneDX(CycloneDXSbom)
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
    pub fn new(
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