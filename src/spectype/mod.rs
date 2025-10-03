pub mod cpu;
pub mod graphics_card;
pub mod cpu_architecture;
pub mod graphics_architecture;
pub mod apu_architecture;
pub mod apu;
pub mod generic_container;
pub mod optional_data;

use async_graphql::{Enum, SimpleObject, Union};
pub use cpu::Cpu;
pub use graphics_card::GraphicsCard;
pub use cpu_architecture::CpuArchitecture;
pub use graphics_architecture::GraphicsArchitecture;
pub use apu_architecture::ApuArchitecture;
pub use apu::Apu;
pub use generic_container::GenericContainer;
pub use optional_data::OptionalData;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::spectype::optional_data::InheritData;


// todo: turn each one of these enum variants into a struct
// easy way to tell which type requires what data, and what data are optional.
#[derive(Clone)]
#[derive(Serialize)]
#[derive(Union)]
pub enum Type {
    Cpu(Cpu),
    Apu(Apu),
    GraphicsCard(GraphicsCard),
    CpuArchitecture(CpuArchitecture),
    ApuArchitecture(ApuArchitecture),
    GraphicsArchitecture(GraphicsArchitecture),
    GenericContainer(GenericContainer),
    Hidden(InheritData)
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
            return Some(Self::CpuArchitecture(CpuArchitecture::from_yaml(&parsed_data)));
        }
        if "APU Architecture".to_string() == label {
            return Some(Self::ApuArchitecture(ApuArchitecture::from_yaml(&parsed_data)));
        }
        if "Graphics Architecture".to_string() == label {
            return Some(Self::GraphicsArchitecture(GraphicsArchitecture::from_yaml(&parsed_data)));
        }
        if "Generic Container".to_string() == label {
            return Some(Self::GenericContainer(GenericContainer::from_yaml(&parsed_data)));
        }
        if "Hidden".to_string() == label {
            return Some(Self::Hidden(InheritData { data: OptionalData { temp: true } }));
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
            return Some(Self::GenericContainer(GenericContainer::from_hashmap(data)));
        }
        if "Hidden".to_string() == label {
            return Some(Self::Hidden(InheritData { data: OptionalData { temp: true } }));
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
            Type::GenericContainer(_) => "Generic Container".to_string(),
            Type::Hidden(_) => "Hidden".to_string(),
        }
    }
}

pub trait SpecDbType {
    fn from_yaml(data: &Yaml) -> Self;
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self;
}
