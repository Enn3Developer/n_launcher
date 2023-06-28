use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AssetIndex {
    id: String,
    sha1: String,
    size: usize,
    total_size: usize,
    url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Artifact {
    sha1: String,
    size: usize,
    url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DownloadClient {
    client: Artifact,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DownloadArtifact {
    artifact: Artifact,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JavaVersion {
    component: String,
    major_version: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Library {
    downloads: DownloadArtifact,
    name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DevData {
    asset_index: AssetIndex,
    assets: String,
    downloads: DownloadClient,
    id: String,
    java_version: JavaVersion,
    libraries: Vec<Library>,
    main_class: String,
    minecraft_arguments: String,
    minimum_launcher_version: usize,
    time: String,
    r#type: String,
}
