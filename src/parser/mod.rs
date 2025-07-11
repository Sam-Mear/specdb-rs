use std::fs;

use hashlink::LinkedHashMap;
use yaml_rust2::{Yaml, YamlLoader};

use crate::{spectype::Type, SpecDb, SpecDbStruct};

#[derive(Clone)]
pub struct SpecDbFile {
    pub file_path: String,
    pub contents: String,
    pub yaml: Yaml,
    pub data: SpecDbStruct
}

impl SpecDbFile {
    // fn from_yaml()
    pub fn from_file_path(file_path: &String) -> Option<SpecDbFile>
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

    pub fn from_file_path_and_inherit(file_path: &String, mut data: LinkedHashMap<String, Yaml>) -> Option<SpecDbFile>
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

pub struct SplitSpecDbFiles {
    pub spec_db_files: Vec<SpecDbFile>,
    pub file_with_inherits: Vec<String>
}
impl SplitSpecDbFiles {
    pub fn new() -> Self
    {
        SplitSpecDbFiles {
            spec_db_files: Vec::<SpecDbFile>::new(),
            file_with_inherits: Vec::<String>::new()
        }
    }
    pub fn merge(&mut self, new: &mut SplitSpecDbFiles)
    {
        self.spec_db_files.append(&mut new.spec_db_files);
        self.file_with_inherits.append(&mut new.file_with_inherits);
    }
    // pub fn push_inherits(&mut self, item: String)
    // {
    //     self.file_with_inherits.push(item);
    // }
    pub fn get_spec_db(&self) -> SpecDb
    {
        let mut merged_with_inherit = Vec::<SpecDbStruct>::new();
        let mut hidden_files = self.get_hidden_types();

        // let mut merged_files: Vec<>
        for file_path in self.file_with_inherits.clone().into_iter() {
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
            let mut data = LinkedHashMap::<String, Yaml>::new();
            let inherits = parsed_data["inherits"].as_vec().expect("Expected array in inherits");
            for inherit_name in inherits {
                let inherit_name = inherit_name.as_str().expect("Inherit name expected to be a string").to_owned();
                // get the inherit object
                let inherit_object = self.find_by_name(&inherit_name.as_str(), Some(&hidden_files))
                                    .expect(format!("Expected a hidden type yaml file with name {}", inherit_name).as_str());
                
                // turn the ['data'] into a hashmap
                let local_data = inherit_object.yaml["data"].as_hash().expect("Failed to create hashmap out of Data");
                // foreach the next inherit object ['data'] hashmap into the above
                for bah in local_data.iter() {
                    data.insert(bah.0.as_str().expect("Value expected as string").to_string(), bah.1.clone());
                }
            }

            // then generate the SpecDbFile from that.
            merged_with_inherit.push(SpecDbFile::from_file_path_and_inherit(&file_path, data).unwrap().data);
            

        }
        for each in &self.spec_db_files {
            merged_with_inherit.push(each.data.clone());
        }

        // merged_with_inherit.append(&mut self.spec_db_files.clone());
        SpecDb { files: merged_with_inherit }
    }

    fn get_hidden_types(&self) -> Vec<SpecDbFile>
    {
        let mut hidden_files = Vec::<SpecDbFile>::new();
        for each in self.spec_db_files.clone().into_iter() {
            if let Type::Hidden(_) = each.data.part_type {
                hidden_files.push(each.clone());
            }
        }
        return hidden_files;
    }

    fn find_by_name(&self, search_term: &str, search_list: Option<&Vec<SpecDbFile>>) -> Option<SpecDbFile>
    {
        let search_list = match search_list {
            Some(search_list) => search_list,
            None => &self.spec_db_files,
        };

        for each in search_list.iter() {
            if each.data.name.as_str() == search_term {
                return Some(each.clone());
            }
        }
        return None;
    }
}
