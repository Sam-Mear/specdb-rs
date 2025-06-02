use std::{ffi::IntoStringError, fs::{self, DirEntry}};
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};
use clap::builder::TypedValueParser;

struct Type {
    name: String
}
struct SpecDbStruct {
    name: String,
    part_type: Type,
    is_part: bool,
    release_date: Option<String>
}

struct SpecDbFile {
    file_path: String,
    contents: String,
    yaml: Yaml,
    data: SpecDbStruct
}

impl SpecDbFile {
    fn from_file_path(file_path: String) -> SpecDbFile
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


        let mut struct_object = SpecDbStruct {
            name: part_name.to_owned(),
            part_type: Type {
                name: part_type
            },
            is_part: is_part,
            release_date: match release_date{
                Some(s) => Some(s.to_string()),
                None => None,
            }
        };


        
        let bah = SpecDbFile {
            file_path: file_path.clone(),
            contents: contents.clone(),
            yaml: YamlLoader::load_from_str(&contents).unwrap()[0].clone(),
            data: struct_object
        };
        return bah;
    }
}

fn main() {
    let bah = list_files("/home/smear/personal/SpecDB/specs".to_string());
    for item in bah {
        println!("File: {}\nFirstLine: {}", item.file_path, item.contents);
    };
}

fn list_files(dir:String) -> Vec<SpecDbFile>
{
    let paths = fs::read_dir(dir).unwrap();
    let mut file_names: Vec<SpecDbFile> = Vec::<SpecDbFile>::new();
    for path in paths {
        match path {
            Ok(path) => file_names.append(&mut check_path(path)),
            Err(error) => print!("Error when getting path: {}",error),
        }
        
    }
    return file_names;
}

fn check_path(path: DirEntry) -> Vec<SpecDbFile>
{
    let mut file_names: Vec<SpecDbFile> = Vec::<SpecDbFile>::new();
    if path.file_type().unwrap().is_dir() {
        file_names.extend(list_files(path.path().as_path().to_str().unwrap().to_string()));
    } else {
        let path_str = path.path().as_path().to_str().unwrap().to_owned();
        if !path_str.ends_with("ignore") && !path_str.ends_with("disable") && !path_str.ends_with(".md") {
            file_names.push(SpecDbFile::from_file_path(path_str));
        }
    }
    return file_names;
}
