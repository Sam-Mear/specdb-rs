use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct GraphicsArchitecture {
    sections: Vec<Section>,
    lithography: Lithography,
    release_date: ReleaseDate,
    direct_x_support: Option<DirectXSupport>,
    vulkan_support: Option<VulkanSupport>,
    manufacturer: Option<Manufacturer>,
}
impl SpecDbType for GraphicsArchitecture {
    fn from_yaml(parsed_data: &Yaml) -> Self {
        let data = parsed_data["data"].clone();// clone temp
        let sections = match Section::from_yaml(parsed_data) {
            Ok(value) => value,
            Err(_) => panic!("error on sections")
        };
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        GraphicsArchitecture {
            sections: sections,
            lithography: Lithography (lithography),
            release_date: ReleaseDate (release_date),
            manufacturer: match (data["Manufacturer"].as_str()) {
                Some(value) => Some( Manufacturer(value.to_string())),
                None => None
            },
            direct_x_support: match data["DirectX Support"].as_str() {
                Some(value) => Some(DirectXSupport(value.to_string())),
                None => None
            },
            vulkan_support: match data["Vulkan Support"].as_str() {
                Some(value) => Some(VulkanSupport(value.to_string())),
                None => None
            },
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        GraphicsArchitecture {
            sections: todo!(),
            lithography: Lithography (lithography),
            release_date: ReleaseDate (release_date),
            manufacturer: match (data.get("Manufacturer")) {
                Some(value) => Some(Manufacturer(value.as_str().expect("Manufacturer must be a string").to_string())),
                None => None
            },
            direct_x_support: data.get("DirectX Support").and_then(|v| v.as_str().map(|s| DirectXSupport(s.to_string()))),
            vulkan_support: data.get("Vulkan Support").and_then(|v| v.as_str().map(|s| VulkanSupport(s.to_string()))),
        }
    }
}