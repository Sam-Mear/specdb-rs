use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct Apu {
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