use crate::technic::TechnicData;
use std::io::Cursor;
use std::path::PathBuf;
use zip::ZipArchive;

pub mod command;
pub mod data;
pub mod technic;

pub fn download(root: &PathBuf, technic_data: &TechnicData) {
    let resp = reqwest::blocking::Client::builder()
        .timeout(None)
        .build()
        .expect("can't create client downloader")
        .get(technic_data.download_url())
        .send()
        .expect("can't download pack")
        .bytes()
        .expect("can't load pack zip as bytes")
        .into_iter()
        .collect::<Vec<u8>>();
    let reader = Cursor::new(resp);
    let mut zip = ZipArchive::new(reader).expect("can't read zip file");
    zip.extract(root).expect("can't extract all files from zip");
}
