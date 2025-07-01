use std::{collections::HashSet, ffi::IntoStringError, fs::{self, DirEntry}, thread::Thread};
use hashlink::LinkedHashMap;
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};
use clap::builder::TypedValueParser;
mod data;
use crate::data::{*};
use specdb_query::SpecDbType;
use specdb_query::spectype::cpu::Cpu;



#[derive(Clone)]
struct CpuArchitecture {
    lithography: Lithography,
    release_date: ReleaseDate,
    sockets: Sockets
}
impl SpecDbType for CpuArchitecture {
    fn from_yaml(data: &Yaml) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        let sockets_yaml = data["Sockets"].as_vec().expect("Sockets is required for Cpu Architecture");
        let mut sockets = Vec::new();
        for socket in sockets_yaml {
            sockets.push(socket.as_str().expect("error in socket array. Could it be coming in as an integer?").to_string());
        }
        CpuArchitecture {
            lithography: Lithography { value: lithography },
            release_date: ReleaseDate { value: release_date },
            sockets: Sockets { value: sockets }
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        let sockets_yaml = data["Sockets"].as_vec().expect("Sockets is required for Cpu Architecture");
        let mut sockets = Vec::new();
        for socket in sockets_yaml {
            sockets.push(socket.as_str().expect("error in socket array. Could it be coming in as an integer?").to_string());
        }
        CpuArchitecture {
            lithography: Lithography { value: lithography },
            release_date: ReleaseDate { value: release_date },
            sockets: Sockets { value: sockets }
        }
    }
}

#[derive(Clone)]
struct GraphicsArchitecture {
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

#[derive(Clone)]
struct ApuArchitecture {
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

#[derive(Clone)]
struct GraphicsCard {
    vram_capacity: VramCapacity,
    shader_processor_count: ShaderProcessorCount,
    gpu_base_frequency: GpuBaseFrequency
}
impl SpecDbType for GraphicsCard {
    fn from_yaml(data: &Yaml) -> Self {
        let vram_capacity = data["VRAM Capacity"].as_str().expect("VRAM Capacity is required for type Graphics Card.").to_string();
        let shader_processor_count = data["Shader Processor Count"].as_i64().expect("Shader Processor Count is required for type Graphics Card");
        let gpu_base_frequency = data["GPU Base Frequency"].as_str().expect("GPU Base Frequency is required for type Graphics Card.").to_string();
        GraphicsCard {
            vram_capacity: VramCapacity { value: vram_capacity },
            shader_processor_count: ShaderProcessorCount { value: u32::try_from(shader_processor_count).expect("Shader processer count too high.") },
            gpu_base_frequency: GpuBaseFrequency { value: gpu_base_frequency }
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        let vram_capacity = data.get("VRAM Capacity")
                .expect("VRAM Capacity is required for type Grahpics Card.")
                .as_str().expect("VRAM Capacity must be string");
        let shader_processor_count = data.get("Shader Processor Count")
                .expect("Shader Processor Count is required for type Graphics Card")
                .as_i64().expect("Shader Processor Count must be int");
        let gpu_base_frequency = data.get("GPU Base Frequency")
                .expect("GPU Base Frequency is required for type Graphics Card")
                .as_str().expect("GPU Base Frequency must be string.").to_string();
        GraphicsCard {
            vram_capacity: VramCapacity { value: vram_capacity.to_string() },
            shader_processor_count: ShaderProcessorCount { value: u32::try_from(shader_processor_count).expect("Shader processer count too high.") },
            gpu_base_frequency: GpuBaseFrequency { value: gpu_base_frequency }
        }
    }
}

#[derive(Clone)]
struct Apu {
    core_count: CoreCount,
    thread_count: ThreadCount,
    base_frequency: BaseFrequency,
    shader_processor_count: ShaderProcessorCount
}
impl SpecDbType for Apu {
    fn from_yaml(data: &Yaml) -> Self {
        let core_count = data["Core Count"].as_i64().expect("Core Count is required for type Apu");
        let thread_count = data["Thread Count"].as_i64().expect("Thread count is required for type Apu");
        let base_frequency = data["Base Frequency"].as_str().expect("Base Frequency is required for type Apu").to_string();
        let shader_processor_count = data["Shader Processor Count"].as_i64().expect("Shader Processor Count is required for type Apu");
        Apu {
            core_count: CoreCount { value: u16::try_from(core_count).expect("Core Count too large") },
            thread_count: ThreadCount { value: u16::try_from(thread_count).expect("Core Count too large") },
            base_frequency: BaseFrequency { value: base_frequency },
            shader_processor_count: ShaderProcessorCount { value: u32::try_from(shader_processor_count).expect("Shader processer count too high.") },
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        let core_count = data.get("Core Count")
                .expect("Core Count is required for type Apu.")
                .as_i64().expect("Core Count must be i64");
        let thread_count = data.get("Thread Count")
                .expect("Thread Count is required for type Apu")
                .as_i64().expect("Thread Count must be i64");
        let base_frequency = data.get("Base Frequency")
                .expect("Base Frequency is required for Apu")
                .as_str().expect("Base Frequency must be string");
        let shader_processor_count = data.get("Shader Processor Count")
                .expect("Shader Proccessor Count is required for type Apu")
                .as_i64().expect("Shader Processor count must be i64");
        
        Apu {
            core_count: CoreCount { value: u16::try_from(core_count).expect("Core Count too large") },
            thread_count: ThreadCount { value: u16::try_from(thread_count).expect("Core Count too large") },
            base_frequency: BaseFrequency { value: base_frequency.to_string() },
            shader_processor_count: ShaderProcessorCount { value: u32::try_from(shader_processor_count).expect("Shader processer count too high.") },
        }
    }
}


// todo: turn each one of these enum variants into a struct
// easy way to tell which type requires what data, and what data are optional.
#[derive(Clone)]
enum Type {
    Cpu(Cpu),
    Apu(Apu),
    GraphicsCard(GraphicsCard),
    CpuArchitecture(CpuArchitecture),
    ApuArchitecture(ApuArchitecture),
    GraphicsArchitecture(GraphicsArchitecture),
    GenericContainer,
    Hidden
}

impl Type {
    pub fn from_yaml(label: String, parsed_data: &Yaml) -> Option<Self>
    {
        if "CPU".to_string() == label {
            return Some(Self::Cpu(Cpu::from_yaml(&parsed_data["data"])));
        }
        else if "APU".to_string() == label {
            return Some(Self::Apu(Apu::from_yaml(&parsed_data["data"])));
        }
        if "Graphics Card".to_string() == label {
            return Some(Self::GraphicsCard(GraphicsCard::from_yaml(&parsed_data["data"])));
        }
        if "CPU Architecture".to_string() == label {
            return Some(Self::CpuArchitecture(CpuArchitecture::from_yaml(&parsed_data["data"])));
        }
        if "APU Architecture".to_string() == label {
            return Some(Self::ApuArchitecture(ApuArchitecture::from_yaml(&parsed_data["data"])));
        }
        if "Graphics Architecture".to_string() == label {
            return Some(Self::GraphicsArchitecture(GraphicsArchitecture::from_yaml(&parsed_data["data"])));
        }
        if "Generic Container".to_string() == label {
            return Some(Self::GenericContainer);
        }
        if "Hidden".to_string() == label {
            return Some(Self::Hidden);
        }
        return None;
    }
    pub fn from_hashmap(label: String, data: LinkedHashMap<String, Yaml>) -> Option<Self>
    {
        if "CPU".to_string() == label {
            return Some(Self::Cpu(Cpu::from_hashmap(data)));
        }
        else if "APU".to_string() == label {
            return Some(Self::Apu(Apu::from_hashmap(data)));
        }
        if "Graphics Card".to_string() == label {
            return Some(Self::GraphicsCard(GraphicsCard::from_hashmap(data)));
        }
        if "CPU Architecture".to_string() == label {
            return Some(Self::CpuArchitecture(CpuArchitecture::from_hashmap(data)));
        }
        if "APU Architecture".to_string() == label {
            return Some(Self::ApuArchitecture(ApuArchitecture::from_hashmap(data)));
        }
        if "Graphics Architecture".to_string() == label {
            return Some(Self::GraphicsArchitecture(GraphicsArchitecture::from_hashmap(data)));
        }
        if "Generic Container".to_string() == label {
            return Some(Self::GenericContainer);
        }
        if "Hidden".to_string() == label {
            return Some(Self::Hidden);
        }
        return None;
    }
    pub fn label(&self) -> String {
        match self {
            Type::Cpu(_) => "CPU".to_string(),
            Type::Apu(_) => "APU".to_string(),
            Type::GraphicsCard(_) => "Graphics Card".to_string(),
            Type::CpuArchitecture(_) => "CPU Architecture".to_string(),
            Type::ApuArchitecture(_) => "APU Architecture".to_string(),
            Type::GraphicsArchitecture(_) => "Graphics Architecture".to_string(),
            Type::GenericContainer => "Generic Container".to_string(),
            Type::Hidden => "Hidden".to_string(),
        }
    }
}

#[derive(Clone)]
struct SpecDbStruct {
    name: String,
    part_type: Type,
    is_part: bool,
}

#[derive(Clone)]
struct SpecDbFile {
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
