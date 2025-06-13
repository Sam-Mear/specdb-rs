use std::{collections::HashSet, ffi::IntoStringError, fs::{self, DirEntry}, thread::Thread};
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};
use clap::builder::TypedValueParser;
mod data;
use crate::data::{*};

trait SpecDbType {
    fn from_data(data: &Yaml) -> Self;
}


struct CpuArchitecture {
    lithography: Lithography,
    release_date: ReleaseDate,
    sockets: Sockets
}
impl SpecDbType for CpuArchitecture {
    fn from_data(data: &Yaml) -> Self {
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

struct GraphicsArchitecture {
    lithography: Lithography,
    release_date: ReleaseDate
}
impl SpecDbType for GraphicsArchitecture {
    fn from_data(data: &Yaml) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        GraphicsArchitecture {
            lithography: Lithography { value: lithography },
            release_date: ReleaseDate { value: release_date }
        }
    }
}

struct ApuArchitecture {
    lithography: Lithography,
    release_date: ReleaseDate
}
impl SpecDbType for ApuArchitecture{
    fn from_data(data: &Yaml) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        ApuArchitecture {
            lithography: Lithography { value: lithography },
            release_date: ReleaseDate { value: release_date }
        }
    }
}

struct Cpu {
    core_count: CoreCount,
    thread_count: ThreadCount,
    base_frequency: BaseFrequency,
    tdp: Tdp
}
impl SpecDbType for Cpu {
    fn from_data(data: &Yaml) -> Self {
        let core_count = data["Core Count"].as_i64().expect("Core Count is required for type Cpu");
        let thread_count = data["Thread Count"].as_i64().expect("Thread count is required for type Cpu");
        let base_frequency = data["Base Frequency"].as_str().expect("Base Frequency is required for type Cpu").to_string();
        let tdp = data["TDP"].as_str().expect("TDP is required for type Cpu").to_string();
        Cpu {
            core_count: CoreCount { value: u16::try_from(core_count).expect("Core Count too large") },
            thread_count: ThreadCount { value: u16::try_from(thread_count).expect("Core Count too large") },
            base_frequency: BaseFrequency { value: base_frequency },
            tdp: Tdp { value: tdp }
        }
    }
}

struct GraphicsCard {
    vram_capacity: VramCapacity,
    shader_processor_count: ShaderProcessorCount,
    gpu_base_frequency: GpuBaseFrequency
}
impl SpecDbType for GraphicsCard {
    fn from_data(data: &Yaml) -> Self {
        let vram_capacity = data["VRAM Capacity"].as_str().expect("VRAM Capacity is required for type Graphics Card.").to_string();
        let shader_processor_count = data["Shader Processor Count"].as_i64().expect("Shader Processor Count is required for type Graphics Card");
        let gpu_base_frequency = data["GPU Base Frequency"].as_str().expect("GPU Base Frequency is required for type Graphics Card.").to_string();
        GraphicsCard {
            vram_capacity: VramCapacity { value: vram_capacity },
            shader_processor_count: ShaderProcessorCount { value: u32::try_from(shader_processor_count).expect("Shader processer count too high.") },
            gpu_base_frequency: GpuBaseFrequency { value: gpu_base_frequency }
        }
    }
}

struct Apu {
    core_count: CoreCount,
    thread_count: ThreadCount,
    base_frequency: BaseFrequency,
    shader_processor_count: ShaderProcessorCount
}
impl SpecDbType for Apu {
    fn from_data(data: &Yaml) -> Self {
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
}


// todo: turn each one of these enum variants into a struct
// easy way to tell which type requires what data, and what data are optional.
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
    pub fn from_label(label: String, parsed_data: &Yaml) -> Option<Self>
    {
        if "CPU".to_string() == label {
            return Some(Self::Cpu(Cpu::from_data(&parsed_data["data"])));
        }
        else if "APU".to_string() == label {
            return Some(Self::Apu(Apu::from_data(&parsed_data["data"])));
        }
        if "Graphics Card".to_string() == label {
            return Some(Self::GraphicsCard(GraphicsCard::from_data(&parsed_data["data"])));
        }
        if "CPU Architecture".to_string() == label {
            return Some(Self::CpuArchitecture(CpuArchitecture::from_data(&parsed_data["data"])));
        }
        if "APU Architecture".to_string() == label {
            return Some(Self::ApuArchitecture(ApuArchitecture::from_data(&parsed_data["data"])));
        }
        if "Graphics Architecture".to_string() == label {
            return Some(Self::GraphicsArchitecture(GraphicsArchitecture::from_data(&parsed_data["data"])));
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

            let mut struct_object = SpecDbStruct {
                name: part_name.to_owned(),
                part_type: Type::from_label(part_type, &parsed_data).expect(format!("Invalid part type in file: {}", file_path).as_str()),
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
            return Some(bah);
        }
        None
    }
}

fn main() {
    let bah = list_files("/home/sam/Documents/code/SpecDB/specs".to_string());
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
    let mut file_with_inherits = Vec::<String>::new();
    if path.file_type().unwrap().is_dir() {
        file_names.extend(list_files(path.path().as_path().to_str().unwrap().to_string()));
    } else {
        let path_str = path.path().as_path().to_str().unwrap().to_owned();
        if !path_str.ends_with("ignore") && !path_str.ends_with("disable") && !path_str.ends_with(".md") {
            match SpecDbFile::from_file_path(&path_str) {
                Some(specDbBah) => file_names.push(specDbBah),
                None => file_with_inherits.push(path_str),
            }
        }
    }
    return file_names;
}
