use hashlink::LinkedHashMap;
use yaml_rust2::Yaml;

use crate::{data::*, SpecDbType};

#[derive(Clone)]
pub struct GraphicsArchitecture {
    lithography: Lithography,
    release_date: ReleaseDate
}
impl SpecDbType for GraphicsArchitecture {
    fn from_yaml(data: &Yaml) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        GraphicsArchitecture {
            lithography: Lithography { value: lithography },
            release_date: ReleaseDate { value: release_date }
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        GraphicsArchitecture {
            lithography: Lithography { value: lithography },
            release_date: ReleaseDate { value: release_date }
        }
    }
}