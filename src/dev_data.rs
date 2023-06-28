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
    artifact: Option<Artifact>,
    classifiers: Option<Classifiers>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Natives {
    path: String,
    sha1: String,
    size: usize,
    url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Classifiers {
    natives_linux: Natives,
    natives_osx: Natives,
    natives_windows: Natives,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JavaVersion {
    component: String,
    major_version: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OS {
    name: String,
    version: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Action {
    action: String,
    os: Option<OS>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Extract {
    exclude: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NativesData {
    linux: String,
    osx: String,
    windows: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Library {
    downloads: DownloadArtifact,
    name: String,
    rules: Option<Vec<Action>>,
    extract: Option<Extract>,
    natives: Option<NativesData>,
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
