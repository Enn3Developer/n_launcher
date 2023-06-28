use serde::{Deserialize, Serialize};

use crate::technic::TechnicData;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Data {
    name: String,
    technic_data: TechnicData,
    ram: u32,
    java: String,
}

impl Data {
    pub fn new(name: String, technic_data: TechnicData, ram: u32, java: String) -> Self {
        Self {
            name,
            technic_data,
            ram,
            java,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn technic_data(&self) -> &TechnicData {
        &self.technic_data
    }

    pub fn ram(&self) -> u32 {
        self.ram
    }

    pub fn java(&self) -> String {
        self.java.clone()
    }
}
