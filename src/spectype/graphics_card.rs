use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct GraphicsCard {
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
            vram_capacity: VramCapacity{ value: vram_capacity},
            shader_processor_count: ShaderProcessorCount{ value: u32::try_from(shader_processor_count).expect("Shader processer count too high.") },
            gpu_base_frequency: GpuBaseFrequency{ value: gpu_base_frequency }
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
            shader_processor_count: ShaderProcessorCount{ value: u32::try_from(shader_processor_count).expect("Shader processer count too high.") },
            gpu_base_frequency: GpuBaseFrequency { value: gpu_base_frequency }
        }
    }
}