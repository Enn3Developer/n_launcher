use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AssetIndex {
    id: String,
    sha1: String,
    size: usize,
    total_size: usize,
    url: String,
}

impl AssetIndex {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn sha1(&self) -> &str {
        &self.sha1
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn total_size(&self) -> usize {
        self.total_size
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Artifact {
    sha1: String,
    size: usize,
    url: String,
}

impl Artifact {
    pub fn sha1(&self) -> &str {
        &self.sha1
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DownloadClient {
    client: Artifact,
}

impl DownloadClient {
    pub fn client(&self) -> &Artifact {
        &self.client
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DownloadArtifact {
    artifact: Option<Artifact>,
    classifiers: Option<Classifiers>,
}

impl DownloadArtifact {
    pub fn artifact(&self) -> &Option<Artifact> {
        &self.artifact
    }
    pub fn classifiers(&self) -> &Option<Classifiers> {
        &self.classifiers
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Natives {
    path: String,
    sha1: String,
    size: usize,
    url: String,
}

impl Natives {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn sha1(&self) -> &str {
        &self.sha1
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Classifiers {
    natives_linux: Natives,
    natives_osx: Natives,
    natives_windows: Natives,
}

impl Classifiers {
    pub fn natives_linux(&self) -> &Natives {
        &self.natives_linux
    }
    pub fn natives_osx(&self) -> &Natives {
        &self.natives_osx
    }
    pub fn natives_windows(&self) -> &Natives {
        &self.natives_windows
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JavaVersion {
    component: String,
    major_version: usize,
}

impl JavaVersion {
    pub fn component(&self) -> &str {
        &self.component
    }
    pub fn major_version(&self) -> usize {
        self.major_version
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OS {
    name: String,
    version: String,
}

impl OS {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Action {
    action: String,
    os: Option<OS>,
}

impl Action {
    pub fn action(&self) -> &str {
        &self.action
    }
    pub fn os(&self) -> &Option<OS> {
        &self.os
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Extract {
    exclude: Vec<String>,
}

impl Extract {
    pub fn exclude(&self) -> &Vec<String> {
        &self.exclude
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NativesData {
    linux: String,
    osx: String,
    windows: String,
}

impl NativesData {
    pub fn linux(&self) -> &str {
        &self.linux
    }
    pub fn osx(&self) -> &str {
        &self.osx
    }
    pub fn windows(&self) -> &str {
        &self.windows
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Library {
    downloads: DownloadArtifact,
    name: String,
    rules: Option<Vec<Action>>,
    extract: Option<Extract>,
    natives: Option<NativesData>,
}

impl Library {
    pub fn downloads(&self) -> &DownloadArtifact {
        &self.downloads
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn rules(&self) -> &Option<Vec<Action>> {
        &self.rules
    }
    pub fn extract(&self) -> &Option<Extract> {
        &self.extract
    }
    pub fn natives(&self) -> &Option<NativesData> {
        &self.natives
    }
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

impl DevData {
    pub fn asset_index(&self) -> &AssetIndex {
        &self.asset_index
    }
    pub fn assets(&self) -> &str {
        &self.assets
    }
    pub fn downloads(&self) -> &DownloadClient {
        &self.downloads
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn java_version(&self) -> &JavaVersion {
        &self.java_version
    }
    pub fn libraries(&self) -> &Vec<Library> {
        &self.libraries
    }
    pub fn main_class(&self) -> &str {
        &self.main_class
    }
    pub fn minecraft_arguments(&self) -> &str {
        &self.minecraft_arguments
    }
    pub fn minimum_launcher_version(&self) -> usize {
        self.minimum_launcher_version
    }
    pub fn time(&self) -> &str {
        &self.time
    }
    pub fn r#type(&self) -> &str {
        &self.r#type
    }
}
