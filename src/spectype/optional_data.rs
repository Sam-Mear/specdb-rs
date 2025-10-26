use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::Serialize;
use yaml_rust2::Yaml;

use crate::spectype::SpecDbType;


#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct InheritData{ pub data: OptionalData }

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct OptionalData {
    pub temp: bool
}
impl SpecDbType for OptionalData {
    fn from_yaml(data: &Yaml) -> Self {
        OptionalData {
            temp: true
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        OptionalData {
            temp: true
        }
    }
}