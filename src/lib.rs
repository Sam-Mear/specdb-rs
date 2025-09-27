use std::fs::{self, DirEntry};

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::{parser::{SpecDbFile, SplitSpecDbFiles}, spectype::Type};

pub mod spectype;
pub mod parser;
pub mod data;

#[derive(Clone)]
#[derive(Serialize)]
pub struct SpecDb {
    pub files: Vec<SpecDbStruct>
}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct SpecDbStruct {
    pub name: String,
    pub part_type: Type,
}

pub fn get_spec_db(path: String) -> SpecDb {
    let bah = parse_spec_db_specs(path);
    let bah2 = bah.get_spec_db();
    println!("Files with inherits: {}", bah.file_with_inherits.iter().count());
    println!("Files without inherits: {}", bah.spec_db_files.iter().count());
    return bah2;
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

fn check_path(path: DirEntry) -> SplitSpecDbFiles
{
    let mut spec_db_files = SplitSpecDbFiles::new();
    if path.file_type().unwrap().is_dir() {
        spec_db_files.merge(&mut read_files(path.path().as_path().to_str().unwrap().to_string()));
    } else {
        let path_str = path.path().as_path().to_str().unwrap().to_owned();
        if !path_str.ends_with("ignore") && !path_str.ends_with("disable") && !path_str.ends_with(".md") {
            match SpecDbFile::from_file_path(&path_str) {
                Some(spec_db_bah) => spec_db_files.spec_db_files.push(spec_db_bah),
                None => {spec_db_files.file_with_inherits.push(path_str)},
            }
        }
    }
    return spec_db_files;
}



