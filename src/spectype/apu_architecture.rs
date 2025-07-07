use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
pub struct ApuArchitecture {
    lithography: Lithography,
    release_date: ReleaseDate
}
impl SpecDbType for ApuArchitecture{
    fn from_yaml(data: &Yaml) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        ApuArchitecture {
            lithography: Lithography { value: lithography },
            release_date: ReleaseDate { value: release_date }
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        let lithography = data.get("Lithography")
                .expect("Lithography is required for Cpu Architecture")
                .as_str().expect("Lithography must be string").to_string();
        let release_date = data.get("Release Date")
                .expect("Release Date is required for Cpu Architecture")
                .as_str().expect("Release Date must be string").to_string();
        ApuArchitecture {
            lithography: Lithography { value: lithography },
            release_date: ReleaseDate { value: release_date }
        }
    }
}