use yaml_rust2::Yaml;
use hashlink::LinkedHashMap;

pub mod spectype;

pub mod data;


pub trait SpecDbType {
    fn from_yaml(data: &Yaml) -> Self;
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self;
}

#[derive(Clone)]
struct SpecDbStruct {
    name: String,
    part_type: Type,
    is_part: bool,
}

#[derive(Clone)]
pub struct SpecDbFile {
    file_path: String,
    contents: String,
    yaml: Yaml,
    data: SpecDbStruct
}

impl SpecDbFile {
    // fn from_yaml()
    fn from_file_path(file_path: &String) -> Option<SpecDbFile>
    {
        let contents = fs::read_to_string(file_path.clone()).unwrap();
        println!("{}",file_path.clone().to_string());
        let parsed_data = YamlLoader::load_from_str(&contents).unwrap()[0].clone();
        let is_part = match parsed_data["isPart"].as_bool() {
            Some(is_part) => is_part,
            None => false,
        };
        let part_type = match parsed_data["type"].as_str() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        let part_name = parsed_data["name"].as_str().expect(format!("Missing required name. Or it is not a string. File: {}", file_path).as_str());
        let release_date = parsed_data["data"]["Release Date"].as_str();
        let inherits = match parsed_data["inherits"].as_vec() {
            Some(_) => true,
            None => false,
        };
        if inherits {
            println!("Ignoring {} for now, will parse when all hidden types are parsed.", part_name)
        } else {

            println!("Parsing {}", part_name);

            let struct_object = SpecDbStruct {
                name: part_name.to_owned(),
                part_type: Type::from_yaml(part_type, &parsed_data).expect(format!("Invalid part type in file: {}", file_path).as_str()),
                is_part: is_part
            };


            
            let bah = SpecDbFile {
                file_path: file_path.clone(),
                contents: contents.clone(),
                yaml: YamlLoader::load_from_str(&contents).unwrap()[0].clone(),
                data: struct_object
            };
            return Some(bah);
        }
        None
    }

    fn from_file_path_and_inherit(file_path: &String, mut data: LinkedHashMap<String, Yaml>) -> Option<SpecDbFile>
    {
        let contents = fs::read_to_string(file_path.clone()).unwrap();
        println!("{}",file_path.clone().to_string());
        let parsed_data = YamlLoader::load_from_str(&contents).unwrap()[0].clone();
        let is_part = match parsed_data["isPart"].as_bool() {
            Some(is_part) => is_part,
            None => false,
        };
        let part_type = match parsed_data["type"].as_str() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        let part_name = parsed_data["name"].as_str().expect(format!("Missing required name. Or it is not a string. File: {}", file_path).as_str());
        let main_data = parsed_data["data"].as_hash().unwrap();
        for data_element in main_data.iter() {
            data.insert(data_element.0.as_str().expect("expected string").to_string(), data_element.1.clone());
        }

        let mut struct_object = SpecDbStruct {
            name: part_name.to_owned(),
            part_type: Type::from_hashmap(part_type, data).expect(format!("Invalid part type in file: {}", file_path).as_str()),
            is_part: is_part
        };
        let bah = SpecDbFile {
            file_path: file_path.clone(),
            contents: contents.clone(),
            yaml: YamlLoader::load_from_str(&contents).unwrap()[0].clone(),
            data: struct_object
        };
        return Some(bah);
    }
}

