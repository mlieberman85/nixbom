// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpdxSchema {
    #[serde(rename = "Document")]
    pub document: Option<Document>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    /// Provide additional information about an SpdxElement.
    pub annotations: Option<Vec<DocumentAnnotation>>,
    pub comment: Option<String>,
    /// One instance is required for each SPDX file produced. It provides the necessary
    /// information for forward and backward compatibility for processing tools.
    #[serde(rename = "creationInfo")]
    pub creation_info: Option<CreationInfo>,
    /// License expression for dataLicense.  Compliance with the SPDX specification includes
    /// populating the SPDX fields therein with data related to such fields ("SPDX-Metadata").
    /// The SPDX specification contains numerous fields where an SPDX document creator may
    /// provide relevant explanatory text in SPDX-Metadata. Without opining on the lawfulness of
    /// "database rights" (in jurisdictions where applicable), such explanatory text is
    /// copyrightable subject matter in most Berne Convention countries. By using the SPDX
    /// specification, or any portion hereof, you hereby agree that any copyright rights (as
    /// determined by your jurisdiction) in any SPDX-Metadata, including without limitation
    /// explanatory text, shall be subject to the terms of the Creative Commons CC0 1.0 Universal
    /// license. For SPDX-Metadata not containing any copyright rights, you hereby agree and
    /// acknowledge that the SPDX-Metadata is provided to you "as-is" and without any
    /// representations or warranties of any kind concerning the SPDX-Metadata, express, implied,
    /// statutory or otherwise, including without limitation warranties of title,
    /// merchantability, fitness for a particular purpose, non-infringement, or the absence of
    /// latent or other defects, accuracy, or the presence or absence of errors, whether or not
    /// discoverable, all to the greatest extent permissible under applicable law.
    #[serde(rename = "dataLicense")]
    pub data_license: Option<String>,
    /// The describesPackage property relates an SpdxDocument to the package which it describes.
    #[serde(rename = "describesPackages")]
    pub describes_packages: Option<Vec<String>>,
    /// Identify any external SPDX documents referenced within this SPDX document.
    #[serde(rename = "externalDocumentRefs")]
    pub external_document_refs: Option<Vec<ExternalDocumentRef>>,
    /// Files referenced in the SPDX document
    pub files: Option<Vec<File>>,
    /// Indicates that a particular ExtractedLicensingInfo was defined in the subject
    /// SpdxDocument.
    #[serde(rename = "hasExtractedLicensingInfos")]
    pub has_extracted_licensing_infos: Option<Vec<HasExtractedLicensingInfo>>,
    /// Identify name of this SpdxElement.
    pub name: Option<String>,
    /// Packages referenced in the SPDX document
    pub packages: Option<Vec<Package>>,
    /// Relationships referenced in the SPDX document
    pub relationships: Option<Vec<Relationship>>,
    /// Reviewed
    pub revieweds: Option<Vec<Reviewed>>,
    /// Snippets referenced in the SPDX document
    pub snippets: Option<Vec<Snippet>>,
    /// Provide a reference number that can be used to understand how to parse and interpret the
    /// rest of the file. It will enable both future changes to the specification and to support
    /// backward compatibility. The version number consists of a major and minor version
    /// indicator. The major field will be incremented when incompatible changes between versions
    /// are made (one or more sections are created, modified or deleted). The minor field will be
    /// incremented when backwards compatible changes are made.
    #[serde(rename = "spdxVersion")]
    pub spdx_version: Option<String>,
}

/// An Annotation is a comment on an SpdxItem by an agent.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentAnnotation {
    /// Identify when the comment was made. This is to be specified according to the combined
    /// date and time in the UTC format, as specified in the ISO 8601 standard.
    #[serde(rename = "annotationDate")]
    pub annotation_date: Option<String>,
    /// Type of the annotation.
    #[serde(rename = "annotationType")]
    pub annotation_type: Option<AnnotationType>,
    /// This field identifies the person, organization or tool that has commented on a file,
    /// package, or the entire document.
    pub annotator: Option<String>,
    pub comment: Option<String>,
}

/// One instance is required for each SPDX file produced. It provides the necessary
/// information for forward and backward compatibility for processing tools.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreationInfo {
    pub comment: Option<String>,
    /// Identify when the SPDX file was originally created. The date is to be specified according
    /// to combined date and time in UTC format as specified in ISO 8601 standard. This field is
    /// distinct from the fields in section 8, which involves the addition of information during
    /// a subsequent review.
    pub created: Option<String>,
    /// Identify who (or what, in the case of a tool) created the SPDX file. If the SPDX file was
    /// created by an individual, indicate the person's name. If the SPDX file was created on
    /// behalf of a company or organization, indicate the entity name. If the SPDX file was
    /// created using a software tool, indicate the name and version for that tool. If multiple
    /// participants or tools were involved, use multiple instances of this field. Person name or
    /// organization name may be designated as “anonymous” if appropriate.
    pub creators: Option<Vec<String>>,
    /// An optional field for creators of the SPDX file to provide the version of the SPDX
    /// License List used when the SPDX file was created.
    #[serde(rename = "licenseListVersion")]
    pub license_list_version: Option<String>,
}

/// Information about an external SPDX document reference including the checksum. This allows
/// for verification of the external references.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalDocumentRef {
    /// A Checksum is value that allows the contents of a file to be authenticated. Even small
    /// changes to the content of the file will change its checksum. This class allows the
    /// results of a variety of checksum and cryptographic message digest algorithms to be
    /// represented.
    pub checksum: Option<ExternalDocumentRefChecksum>,
    /// externalDocumentId is a string containing letters, numbers, ., - and/or + which uniquely
    /// identifies an external document within this document.
    #[serde(rename = "externalDocumentId")]
    pub external_document_id: Option<String>,
    /// SPDX ID for SpdxDocument.  A propoerty containing an SPDX document.
    #[serde(rename = "spdxDocument")]
    pub spdx_document: Option<String>,
}

/// A Checksum is value that allows the contents of a file to be authenticated. Even small
/// changes to the content of the file will change its checksum. This class allows the
/// results of a variety of checksum and cryptographic message digest algorithms to be
/// represented.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalDocumentRefChecksum {
    /// Identifies the algorithm used to produce the subject Checksum. Currently, SHA-1 is the
    /// only supported algorithm. It is anticipated that other algorithms will be supported at a
    /// later time.
    pub algorithm: Option<Algorithm>,
    /// The checksumValue property provides a lower case hexidecimal encoded digest value
    /// produced using a specific algorithm.
    #[serde(rename = "checksumValue")]
    pub checksum_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    /// Provide additional information about an SpdxElement.
    pub annotations: Option<Vec<FileAnnotation>>,
    /// Indicates the project in which the SpdxElement originated. Tools must preserve
    /// doap:homepage and doap:name properties and the URI (if one is known) of doap:Project
    /// resources that are values of this property. All other properties of doap:Projects are not
    /// directly supported by SPDX and may be dropped when translating to or from some SPDX
    /// formats.
    #[serde(rename = "artifactOfs")]
    pub artifact_ofs: Option<Vec<HashMap<String, Option<serde_json::Value>>>>,
    /// This field provides a place for the SPDX data creator to record acknowledgements that may
    /// be required to be communicated in some contexts. This is not meant to include theactual
    /// complete license text (see licenseConculded and licenseDeclared), and may or may not
    /// include copyright notices (see also copyrightText). The SPDX data creator may use this
    /// field to record other acknowledgements, such as particular clauses from license texts,
    /// which may be necessary or desirable to reproduce.
    #[serde(rename = "attributionTexts")]
    pub attribution_texts: Option<Vec<String>>,
    /// The checksum property provides a mechanism that can be used to verify that the contents
    /// of a File or Package have not changed.
    pub checksums: Option<Vec<FileChecksum>>,
    pub comment: Option<String>,
    /// The text of copyright declarations recited in the Package or File.
    #[serde(rename = "copyrightText")]
    pub copyright_text: Option<String>,
    /// This field provides a place for the SPDX file creator to record file contributors.
    /// Contributors could include names of copyright holders and/or authors who may not be
    /// copyright holders yet contributed to the file content.
    #[serde(rename = "fileContributors")]
    pub file_contributors: Option<Vec<String>>,
    #[serde(rename = "fileDependencies")]
    pub file_dependencies: Option<Vec<String>>,
    /// The name of the file relative to the root of the package.
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    /// The type of the file.
    #[serde(rename = "fileTypes")]
    pub file_types: Option<Vec<FileType>>,
    /// The licenseComments property allows the preparer of the SPDX document to describe why the
    /// licensing in spdx:licenseConcluded was chosen.
    #[serde(rename = "licenseComments")]
    pub license_comments: Option<String>,
    /// The licensing information that was discovered directly within the package. There will be
    /// an instance of this property for each distinct value of alllicenseInfoInFile properties
    /// of all files contained in the package.
    #[serde(rename = "licenseInfoFromFiles")]
    pub license_info_from_files: Option<Vec<String>>,
    /// Licensing information that was discovered directly in the subject file. This is also
    /// considered a declared license for the file.
    #[serde(rename = "licenseInfoInFiles")]
    pub license_info_in_files: Option<Vec<String>>,
    /// Identify name of this SpdxElement.
    pub name: Option<String>,
    /// This field provides a place for the SPDX file creator to record potential legal notices
    /// found in the file. This may or may not include copyright statements.
    #[serde(rename = "noticeText")]
    pub notice_text: Option<String>,
}

/// An Annotation is a comment on an SpdxItem by an agent.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileAnnotation {
    /// Identify when the comment was made. This is to be specified according to the combined
    /// date and time in the UTC format, as specified in the ISO 8601 standard.
    #[serde(rename = "annotationDate")]
    pub annotation_date: Option<String>,
    /// Type of the annotation.
    #[serde(rename = "annotationType")]
    pub annotation_type: Option<AnnotationType>,
    /// This field identifies the person, organization or tool that has commented on a file,
    /// package, or the entire document.
    pub annotator: Option<String>,
    pub comment: Option<String>,
}

/// A Checksum is value that allows the contents of a file to be authenticated. Even small
/// changes to the content of the file will change its checksum. This class allows the
/// results of a variety of checksum and cryptographic message digest algorithms to be
/// represented.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileChecksum {
    /// Identifies the algorithm used to produce the subject Checksum. Currently, SHA-1 is the
    /// only supported algorithm. It is anticipated that other algorithms will be supported at a
    /// later time.
    pub algorithm: Option<Algorithm>,
    /// The checksumValue property provides a lower case hexidecimal encoded digest value
    /// produced using a specific algorithm.
    #[serde(rename = "checksumValue")]
    pub checksum_value: Option<String>,
}

/// An ExtractedLicensingInfo represents a license or licensing notice that was found in the
/// package. Any license text that is recognized as a license may be represented as a License
/// rather than an ExtractedLicensingInfo.
#[derive(Debug, Serialize, Deserialize)]
pub struct HasExtractedLicensingInfo {
    pub comment: Option<String>,
    /// Verbatim license or licensing notice text that was discovered.
    #[serde(rename = "extractedText")]
    pub extracted_text: Option<String>,
    /// A human readable short form license identifier for a license. The license ID is iether on
    /// the standard license oist or the form "LicenseRef-"[idString] where [idString] is a
    /// unique string containing letters, numbers, ".", "-" or "+".
    #[serde(rename = "licenseId")]
    pub license_id: Option<String>,
    /// Identify name of this SpdxElement.
    pub name: Option<String>,
    #[serde(rename = "seeAlsos")]
    pub see_alsos: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    /// Provide additional information about an SpdxElement.
    pub annotations: Option<Vec<PackageAnnotation>>,
    /// This field provides a place for the SPDX data creator to record acknowledgements that may
    /// be required to be communicated in some contexts. This is not meant to include theactual
    /// complete license text (see licenseConculded and licenseDeclared), and may or may not
    /// include copyright notices (see also copyrightText). The SPDX data creator may use this
    /// field to record other acknowledgements, such as particular clauses from license texts,
    /// which may be necessary or desirable to reproduce.
    #[serde(rename = "attributionTexts")]
    pub attribution_texts: Option<Vec<String>>,
    /// The checksum property provides a mechanism that can be used to verify that the contents
    /// of a File or Package have not changed.
    pub checksums: Option<Vec<PackageChecksum>>,
    pub comment: Option<String>,
    /// The text of copyright declarations recited in the Package or File.
    #[serde(rename = "copyrightText")]
    pub copyright_text: Option<String>,
    /// Provides a detailed description of the package.
    pub description: Option<String>,
    /// The URI at which this package is available for download. Private (i.e., not publicly
    /// reachable) URIs are acceptable as values of this property. The values
    /// http://spdx.org/rdf/terms#none and http://spdx.org/rdf/terms#noassertion may be used to
    /// specify that the package is not downloadable or that no attempt was made to determine its
    /// download location, respectively.
    #[serde(rename = "downloadLocation")]
    pub download_location: Option<String>,
    /// An External Reference allows a Package to reference an external source of additional
    /// information, metadata, enumerations, asset identifiers, or downloadable content believed
    /// to be relevant to the Package.
    #[serde(rename = "externalRefs")]
    pub external_refs: Option<Vec<ExternalRef>>,
    /// Indicates whether the file content of this package has been available for or subjected to
    /// analysis when creating the SPDX document. If false indicates packages that represent
    /// metadata or URI references to a project, product, artifact, distribution or a component.
    /// If set to false, the package must not contain any files.
    #[serde(rename = "filesAnalyzed")]
    pub files_analyzed: Option<bool>,
    /// Indicates that a particular file belongs to a package.
    #[serde(rename = "hasFiles")]
    pub has_files: Option<Vec<String>>,
    pub homepage: Option<String>,
    /// The licenseComments property allows the preparer of the SPDX document to describe why the
    /// licensing in spdx:licenseConcluded was chosen.
    #[serde(rename = "licenseComments")]
    pub license_comments: Option<String>,
    /// The licensing information that was discovered directly within the package. There will be
    /// an instance of this property for each distinct value of alllicenseInfoInFile properties
    /// of all files contained in the package.
    #[serde(rename = "licenseInfoFromFiles")]
    pub license_info_from_files: Option<Vec<String>>,
    /// Identify name of this SpdxElement.
    pub name: Option<String>,
    /// The name and, optionally, contact information of the person or organization that
    /// originally created the package. Values of this property must conform to the agent and
    /// tool syntax.
    pub originator: Option<String>,
    /// The base name of the package file name. For example, zlib-1.2.5.tar.gz.
    #[serde(rename = "packageFileName")]
    pub package_file_name: Option<String>,
    /// A manifest based verification code (the algorithm is defined in section 4.7 of the full
    /// specification) of the SPDX Item. This allows consumers of this data and/or database to
    /// determine if an SPDX item they have in hand is identical to the SPDX item from which the
    /// data was produced. This algorithm works even if the SPDX document is included in the SPDX
    /// item.
    #[serde(rename = "packageVerificationCode")]
    pub package_verification_code: Option<PackageVerificationCode>,
    /// Allows the producer(s) of the SPDX document to describe how the package was acquired
    /// and/or changed from the original source.
    #[serde(rename = "sourceInfo")]
    pub source_info: Option<String>,
    /// Provides a short description of the package.
    pub summary: Option<String>,
    /// The name and, optionally, contact information of the person or organization who was the
    /// immediate supplier of this package to the recipient. The supplier may be different than
    /// originator when the software has been repackaged. Values of this property must conform to
    /// the agent and tool syntax.
    pub supplier: Option<String>,
    /// Provides an indication of the version of the package that is described by this
    /// SpdxDocument.
    #[serde(rename = "versionInfo")]
    pub version_info: Option<String>,
}

/// An Annotation is a comment on an SpdxItem by an agent.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageAnnotation {
    /// Identify when the comment was made. This is to be specified according to the combined
    /// date and time in the UTC format, as specified in the ISO 8601 standard.
    #[serde(rename = "annotationDate")]
    pub annotation_date: Option<String>,
    /// Type of the annotation.
    #[serde(rename = "annotationType")]
    pub annotation_type: Option<AnnotationType>,
    /// This field identifies the person, organization or tool that has commented on a file,
    /// package, or the entire document.
    pub annotator: Option<String>,
    pub comment: Option<String>,
}

/// A Checksum is value that allows the contents of a file to be authenticated. Even small
/// changes to the content of the file will change its checksum. This class allows the
/// results of a variety of checksum and cryptographic message digest algorithms to be
/// represented.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageChecksum {
    /// Identifies the algorithm used to produce the subject Checksum. Currently, SHA-1 is the
    /// only supported algorithm. It is anticipated that other algorithms will be supported at a
    /// later time.
    pub algorithm: Option<Algorithm>,
    /// The checksumValue property provides a lower case hexidecimal encoded digest value
    /// produced using a specific algorithm.
    #[serde(rename = "checksumValue")]
    pub checksum_value: Option<String>,
}

/// An External Reference allows a Package to reference an external source of additional
/// information, metadata, enumerations, asset identifiers, or downloadable content believed
/// to be relevant to the Package.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalRef {
    pub comment: Option<String>,
    /// Category for the external reference
    #[serde(rename = "referenceCategory")]
    pub reference_category: Option<ReferenceCategory>,
    /// The unique string with no spaces necessary to access the package-specific information,
    /// metadata, or content within the target location. The format of the locator is subject to
    /// constraints defined by the <type>.
    #[serde(rename = "referenceLocator")]
    pub reference_locator: Option<String>,
    /// Type of the external reference. These are definined in an appendix in the SPDX
    /// specification.
    #[serde(rename = "referenceType")]
    pub reference_type: Option<String>,
}

/// A manifest based verification code (the algorithm is defined in section 4.7 of the full
/// specification) of the SPDX Item. This allows consumers of this data and/or database to
/// determine if an SPDX item they have in hand is identical to the SPDX item from which the
/// data was produced. This algorithm works even if the SPDX document is included in the SPDX
/// item.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageVerificationCode {
    /// A file that was excluded when calculating the package verification code. This is usually
    /// a file containing SPDX data regarding the package. If a package contains more than one
    /// SPDX file all SPDX files must be excluded from the package verification code. If this is
    /// not done it would be impossible to correctly calculate the verification codes in both
    /// files.
    #[serde(rename = "packageVerificationCodeExcludedFiles")]
    pub package_verification_code_excluded_files: Option<Vec<String>>,
    /// The actual package verification code as a hex encoded value.
    #[serde(rename = "packageVerificationCodeValue")]
    pub package_verification_code_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub comment: Option<String>,
    /// SPDX ID for SpdxElement.  A related SpdxElement.
    #[serde(rename = "relatedSpdxElement")]
    pub related_spdx_element: Option<String>,
    /// Describes the type of relationship between two SPDX elements.
    #[serde(rename = "relationshipType")]
    pub relationship_type: Option<RelationshipType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reviewed {
    pub comment: Option<String>,
    /// The date and time at which the SpdxDocument was reviewed. This value must be in UTC and
    /// have 'Z' as its timezone indicator.
    #[serde(rename = "reviewDate")]
    pub review_date: Option<String>,
    /// The name and, optionally, contact information of the person who performed the review.
    /// Values of this property must conform to the agent and tool syntax.
    pub reviewer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    /// Provide additional information about an SpdxElement.
    pub annotations: Option<Vec<SnippetAnnotation>>,
    /// This field provides a place for the SPDX data creator to record acknowledgements that may
    /// be required to be communicated in some contexts. This is not meant to include theactual
    /// complete license text (see licenseConculded and licenseDeclared), and may or may not
    /// include copyright notices (see also copyrightText). The SPDX data creator may use this
    /// field to record other acknowledgements, such as particular clauses from license texts,
    /// which may be necessary or desirable to reproduce.
    #[serde(rename = "attributionTexts")]
    pub attribution_texts: Option<Vec<String>>,
    pub comment: Option<String>,
    /// The text of copyright declarations recited in the Package or File.
    #[serde(rename = "copyrightText")]
    pub copyright_text: Option<String>,
    /// The licenseComments property allows the preparer of the SPDX document to describe why the
    /// licensing in spdx:licenseConcluded was chosen.
    #[serde(rename = "licenseComments")]
    pub license_comments: Option<String>,
    /// The licensing information that was discovered directly within the package. There will be
    /// an instance of this property for each distinct value of alllicenseInfoInFile properties
    /// of all files contained in the package.
    #[serde(rename = "licenseInfoFromFiles")]
    pub license_info_from_files: Option<Vec<String>>,
    /// Licensing information that was discovered directly in the subject snippet. This is also
    /// considered a declared license for the snippet.
    #[serde(rename = "licenseInfoInSnippets")]
    pub license_info_in_snippets: Option<Vec<String>>,
    /// Identify name of this SpdxElement.
    pub name: Option<String>,
    /// This field defines the byte range in the original host file (in X.2) that the snippet
    /// information applies to
    pub ranges: Option<Vec<Range>>,
    /// SPDX ID for File.  File containing the SPDX element (e.g. the file contaning a snippet).
    #[serde(rename = "snippetFromFile")]
    pub snippet_from_file: Option<String>,
}

/// An Annotation is a comment on an SpdxItem by an agent.
#[derive(Debug, Serialize, Deserialize)]
pub struct SnippetAnnotation {
    /// Identify when the comment was made. This is to be specified according to the combined
    /// date and time in the UTC format, as specified in the ISO 8601 standard.
    #[serde(rename = "annotationDate")]
    pub annotation_date: Option<String>,
    /// Type of the annotation.
    #[serde(rename = "annotationType")]
    pub annotation_type: Option<AnnotationType>,
    /// This field identifies the person, organization or tool that has commented on a file,
    /// package, or the entire document.
    pub annotator: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "endPointer")]
    pub end_pointer: Option<EndPointer>,
    #[serde(rename = "startPointer")]
    pub start_pointer: Option<StartPointer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndPointer {
    /// SPDX ID for File
    pub reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartPointer {
    /// SPDX ID for File
    pub reference: Option<String>,
}

/// Type of the annotation.
#[derive(Debug, Serialize, Deserialize)]
pub enum AnnotationType {
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "REVIEW")]
    Review,
}

/// Identifies the algorithm used to produce the subject Checksum. Currently, SHA-1 is the
/// only supported algorithm. It is anticipated that other algorithms will be supported at a
/// later time.
#[derive(Debug, Serialize, Deserialize)]
pub enum Algorithm {
    #[serde(rename = "MD2")]
    Md2,
    #[serde(rename = "MD4")]
    Md4,
    #[serde(rename = "MD5")]
    Md5,
    #[serde(rename = "MD6")]
    Md6,
    #[serde(rename = "SHA1")]
    Sha1,
    #[serde(rename = "SHA224")]
    Sha224,
    #[serde(rename = "SHA256")]
    Sha256,
    #[serde(rename = "SHA384")]
    Sha384,
    #[serde(rename = "SHA512")]
    Sha512,
}

/// The type of the file.
#[derive(Debug, Serialize, Deserialize)]
pub enum FileType {
    #[serde(rename = "APPLICATION")]
    Application,
    #[serde(rename = "ARCHIVE")]
    Archive,
    #[serde(rename = "AUDIO")]
    Audio,
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "DOCUMENTATION")]
    Documentation,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "SOURCE")]
    Source,
    #[serde(rename = "SPDX")]
    Spdx,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "VIDEO")]
    Video,
}

/// Category for the external reference
#[derive(Debug, Serialize, Deserialize)]
pub enum ReferenceCategory {
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "PACKAGE_MANAGER")]
    PackageManager,
    #[serde(rename = "SECURITY")]
    Security,
}

/// Describes the type of relationship between two SPDX elements.
#[derive(Debug, Serialize, Deserialize)]
pub enum RelationshipType {
    #[serde(rename = "ANCESTOR_OF")]
    AncestorOf,
    #[serde(rename = "BUILD_DEPENDENCY_OF")]
    BuildDependencyOf,
    #[serde(rename = "BUILD_TOOL_OF")]
    BuildToolOf,
    #[serde(rename = "CONTAINED_BY")]
    ContainedBy,
    #[serde(rename = "CONTAINS")]
    Contains,
    #[serde(rename = "COPY_OF")]
    CopyOf,
    #[serde(rename = "DATA_FILE_OF")]
    DataFileOf,
    #[serde(rename = "DEPENDENCY_MANIFEST_OF")]
    DependencyManifestOf,
    #[serde(rename = "DEPENDENCY_OF")]
    DependencyOf,
    #[serde(rename = "DEPENDS_ON")]
    DependsOn,
    #[serde(rename = "DESCENDANT_OF")]
    DescendantOf,
    #[serde(rename = "DESCRIBED_BY")]
    DescribedBy,
    #[serde(rename = "DESCRIBES")]
    Describes,
    #[serde(rename = "DEV_DEPENDENCY_OF")]
    DevDependencyOf,
    #[serde(rename = "DEV_TOOL_OF")]
    DevToolOf,
    #[serde(rename = "DISTRIBUTION_ARTIFACT")]
    DistributionArtifact,
    #[serde(rename = "DOCUMENTATION_OF")]
    DocumentationOf,
    #[serde(rename = "DYNAMIC_LINK")]
    DynamicLink,
    #[serde(rename = "EXAMPLE_OF")]
    ExampleOf,
    #[serde(rename = "EXPANDED_FROM_ARCHIVE")]
    ExpandedFromArchive,
    #[serde(rename = "FILE_ADDED")]
    FileAdded,
    #[serde(rename = "FILE_DELETED")]
    FileDeleted,
    #[serde(rename = "FILE_MODIFIED")]
    FileModified,
    #[serde(rename = "GENERATED_FROM")]
    GeneratedFrom,
    #[serde(rename = "GENERATES")]
    Generates,
    #[serde(rename = "HAS_PREREQUISITE")]
    HasPrerequisite,
    #[serde(rename = "METAFILE_OF")]
    MetafileOf,
    #[serde(rename = "OPTIONAL_COMPONENT_OF")]
    OptionalComponentOf,
    #[serde(rename = "OPTIONAL_DEPENDENCY_OF")]
    OptionalDependencyOf,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "PACKAGE_OF")]
    PackageOf,
    #[serde(rename = "PATCH_APPLIED")]
    PatchApplied,
    #[serde(rename = "PATCH_FOR")]
    PatchFor,
    #[serde(rename = "PREREQUISITE_FOR")]
    PrerequisiteFor,
    #[serde(rename = "PROVIDED_DEPENDENCY_OF")]
    ProvidedDependencyOf,
    #[serde(rename = "RUNTIME_DEPENDENCY_OF")]
    RuntimeDependencyOf,
    #[serde(rename = "STATIC_LINK")]
    StaticLink,
    #[serde(rename = "TEST_CASE_OF")]
    TestCaseOf,
    #[serde(rename = "TEST_DEPENDENCY_OF")]
    TestDependencyOf,
    #[serde(rename = "TEST_OF")]
    TestOf,
    #[serde(rename = "TEST_TOOL_OF")]
    TestToolOf,
    #[serde(rename = "VARIANT_OF")]
    VariantOf,
}
