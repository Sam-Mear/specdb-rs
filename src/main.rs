use std::{collections::HashSet, ffi::IntoStringError, fs::{self, DirEntry}, thread::Thread};
use hashlink::LinkedHashMap;
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};
use clap::builder::TypedValueParser;
mod data;
use crate::data::{*};
use specdb_query::SpecDbType;
use specdb_query::spectype::{*};



fn main() {
    let bah = parse_spec_db_specs("/home/smear/personal/SpecDB/specs".to_string());
    println!("Files with inherits: {}", bah.file_with_inherits.iter().count());
    println!("Files without inherits: {}", bah.spec_db_files.iter().count());
    let bah2 = bah.get_spec_db();
    println!("Files with inherits: {}", bah.file_with_inherits.iter().count());
    println!("Files without inherits: {}", bah.spec_db_files.iter().count());
    println!("Files parsed total: {}", bah2.files.iter().count());
}

fn parse_spec_db_specs(dir: String) -> SplitSpecDbFiles
{
    read_files(dir)
}

fn read_files(dir:String) -> SplitSpecDbFiles
{
    let paths = fs::read_dir(dir).unwrap();
    let mut spec_db_files:SplitSpecDbFiles = SplitSpecDbFiles::new();
    for path in paths {
        match path {
            Ok(path) => spec_db_files.merge(&mut check_path(path)),
            Err(error) => print!("Error when getting path: {}",error),
        }
    }
    return spec_db_files;
}

struct SpecDb {
    files: Vec<SpecDbFile>
}

struct SplitSpecDbFiles {
    spec_db_files: Vec<SpecDbFile>,
    file_with_inherits: Vec<String>
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
        let mut merged_with_inherit = Vec::<SpecDbFile>::new();
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
            merged_with_inherit.push(SpecDbFile::from_file_path_and_inherit(&file_path, data).unwrap());
            

        }

        merged_with_inherit.append(&mut self.spec_db_files.clone());
        SpecDb { files: merged_with_inherit }
    }

    fn get_hidden_types(&self) -> Vec<SpecDbFile>
    {
        let mut hidden_files = Vec::<SpecDbFile>::new();
        for each in self.spec_db_files.clone().into_iter() {
            if let Type::Hidden = each.data.part_type {
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

fn check_path(path: DirEntry) -> SplitSpecDbFiles
{
    let mut spec_db_files = SplitSpecDbFiles::new();
    if path.file_type().unwrap().is_dir() {
        spec_db_files.merge(&mut read_files(path.path().as_path().to_str().unwrap().to_string()));
    } else {
        let path_str = path.path().as_path().to_str().unwrap().to_owned();
        if !path_str.ends_with("ignore") && !path_str.ends_with("disable") && !path_str.ends_with(".md") {
            match SpecDbFile::from_file_path(&path_str) {
                Some(specDbBah) => spec_db_files.spec_db_files.push(specDbBah),
                None => {spec_db_files.file_with_inherits.push(path_str)},
            }
        }
    }
    return spec_db_files;
}
