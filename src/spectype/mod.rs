pub mod cpu;
pub mod graphics_card;
pub mod cpu_architecture;
pub mod graphics_architecture;
pub mod apu_architecture;
pub mod apu;

pub use cpu::Cpu;
pub use graphics_card::GraphicsCard;
pub use cpu_architecture::CpuArchitecture;
pub use graphics_architecture::GraphicsArchitecture;
pub use apu_architecture::ApuArchitecture;
pub use apu::Apu;
use hashlink::LinkedHashMap;
use yaml_rust2::Yaml;

use crate::SpecDbType;

// todo: turn each one of these enum variants into a struct
// easy way to tell which type requires what data, and what data are optional.
#[derive(Clone)]
pub enum Type {
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
