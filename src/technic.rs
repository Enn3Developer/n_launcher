use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Feed {
    user: String,
    date: usize,
    content: String,
    avatar: String,
    url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Icon {
    url: String,
    md5: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Response {
    status: Option<u32>,
    error: Option<String>,
    id: Option<u32>,
    name: Option<String>,
    display_name: Option<String>,
    user: Option<String>,
    url: Option<String>,
    platform_url: Option<String>,
    minecraft: Option<String>,
    ratings: Option<u32>,
    installs: Option<u32>,
    runs: Option<u32>,
    description: Option<String>,
    tags: Option<String>,
    is_server: Option<bool>,
    is_official: Option<bool>,
    version: Option<String>,
    force_dir: Option<bool>,
    feed: Option<Vec<Feed>>,
    icon: Option<Icon>,
    logo: Option<Icon>,
    background: Option<Icon>,
    solder: Option<String>,
    discord_server_id: Option<String>,
    server_pack_url: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TechnicData {
    version: String,
    download_url: String,
    icon_url: String,
}

impl TechnicData {
    pub fn new(version: String, download_url: String, icon_url: String) -> Self {
        Self {
            version,
            download_url,
            icon_url,
        }
    }

    pub fn needs_update(&self, other: &Self) -> bool {
        self.version != other.version
    }

    pub fn download_url(&self) -> &str {
        &self.download_url
    }

    pub fn icon_url(&self) -> &str {
        &self.icon_url
    }
}

pub struct Technic {
    pack: String,
}

impl Technic {
    pub fn new(pack: String) -> Self {
        Self { pack }
    }

    pub fn get_data(&self) -> TechnicData {
        let url = format!(
            "https://api.technicpack.net/modpack/{}?build=69420",
            self.pack
        );
        let resp = if let Ok(resp) = reqwest::blocking::get(url) {
            resp
        } else {
            println!("pack {} errored; trying default pack", self.pack);
            reqwest::blocking::get("https://api.technicpack.net/modpack/rgbcraft-test?build=69420")
                .expect("can't get default pack data")
        }
        .json::<Response>()
        .expect("can't convert response to json");
        if let Some(error) = resp.error {
            if let Some(status) = resp.status {
                if status == 401 {
                    panic!("build 69420 not supported anymore");
                }
                panic!("error status: {status}; message: {error}");
            }
            panic!("error message: {error}")
        }

        let version = resp
            .version
            .unwrap_or_else(|| panic!("can't get version from technic data, pack {}", self.pack));

        let download_url = resp
            .url
            .unwrap_or_else(|| panic!("can't get url from technic data, pack {}", self.pack));

        let icon_url = resp
            .icon
            .unwrap_or_else(|| panic!("can't get icon url from technic data, pack {}", self.pack))
            .url;

        TechnicData::new(version, download_url, icon_url)
    }
}
