use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};


#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct GenericContainer{
    pub top_header: String,
    pub sections: Vec<Section>,
}

impl SpecDbType for GenericContainer {
    fn from_yaml(data: &Yaml) -> Self {
        let sections = match Section::from_yaml(data) {
            Ok(value) => value,
            Err(_) => panic!("error on sections")
        };
        let top_header = data["topHeader"].as_str().expect("TopHeader is required for Generic Container").to_string();
        GenericContainer { sections: sections, top_header: top_header }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        GenericContainer { sections: todo!(), top_header: todo!() }
    }
}