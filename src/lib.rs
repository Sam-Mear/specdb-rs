use yaml_rust2::Yaml;
use hashlink::LinkedHashMap;

pub mod spectype;

pub mod data;


pub trait SpecDbType {
    fn from_yaml(data: &Yaml) -> Self;
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self;
}

