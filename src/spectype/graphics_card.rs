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
    gpu_base_frequency: GpuBaseFrequency,
    manufacturer: Option<Manufacturer>,
    vendor: Option<Vendor>,
    architecture: Option<Architecture>,
    lithography: Option<Lithography>,
    release_date: Option<ReleaseDate>,
    // direct_x_support: Option<DirectXSupport>,
    // open_gl_support: Option<OpenGLSupport>,
    // open_cl_support: Option<OpenCLSupport>,
    // vulkan_support: Option<VulkanSupport>,
    // vram_frequency: Option<VramFrequency>,
    // vram_type: Option<VramType>,
    // vram_bandwidth: Option<VramBandwidth>,
    // vram_bus_width: Option<VramBusWidth>,
    // render_output_unit_count: Option<RenderOutputUnitCount>,
    // texture_mapping_unit_count: Option<TextureMappingUnitCount>,
    // die_size: Option<DieSize>,
    tdp: Option<Tdp>,
    // gpu: Option<Gpu>,
    // gpu_variant: Option<GpuVariant>,
    // fp32_compute: Option<FP32Compute>,
    // fp64_compute: Option<FP64Compute>,
    // slot_width: Option<SlotWidth>,
    // outputs: Option<Outputs>,
    // power_connectors: Option<PowerConnectors>,
    // length: Option<Length>,
    // height: Option<Height>,
    // width: Option<Width>,
    ray_tracing_cores: Option<RayTracingCores>,
    tensor_cores: Option<TensorCores>,
}
impl SpecDbType for GraphicsCard {
    fn from_yaml(data: &Yaml) -> Self {
        let vram_capacity = data["VRAM Capacity"].as_str().expect("VRAM Capacity is required for type Graphics Card.").to_string();
        let shader_processor_count = data["Shader Processor Count"].as_i64().expect("Shader Processor Count is required for type Graphics Card");
        let gpu_base_frequency = data["GPU Base Frequency"].as_str().expect("GPU Base Frequency is required for type Graphics Card.").to_string();
        GraphicsCard {
            vram_capacity: VramCapacity(vram_capacity),
            shader_processor_count: ShaderProcessorCount(u32::try_from(shader_processor_count).expect("Shader processer count too high.") ),
            gpu_base_frequency: GpuBaseFrequency(gpu_base_frequency ),
            manufacturer: match (data["Manufacturer"].as_str()) {
                Some(value) => Some( Manufacturer(value.to_string())),
                None => None
            },
            vendor: match (data["Vendor"].as_str()) {
                Some(value) => Some(Vendor(value.to_string())),
                None => None
            },
            architecture: match (data["Architecture"].as_str()) {
                Some(value) => Some(Architecture(value.to_string())),
                None => None
            },
            lithography: match (data["Lithography"].as_str()) {
                Some(value) => Some(Lithography(value.to_string())),
                None => None
            },
            release_date: match (data["Release Date"].as_str()) {
                Some(value) => Some(ReleaseDate(value.to_string())),
                None => None
            },
            tdp: match (data["TDP"].as_str()) {
                Some(value) => Some(Tdp(value.to_string())),
                None => None
            },
            ray_tracing_cores: match (data["Ray Tracing Cores"].as_i64()) {
                Some(value) => Some(RayTracingCores(u16::try_from(value).expect("Ray Tracing Cores does not fit in u16"))),
                None => None
            },
            tensor_cores: match (data["Tensor Cores"].as_i64()) {
                Some(value) => Some(TensorCores(u16::try_from(value).expect("Tensor Cores does not fit into u16"))),
                None => None
            },
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
            vram_capacity: VramCapacity (vram_capacity.to_string()),
            shader_processor_count: ShaderProcessorCount(u32::try_from(shader_processor_count).expect("Shader processer count too high.")),
            gpu_base_frequency: GpuBaseFrequency (gpu_base_frequency),
            manufacturer: match (data.get("Manufacturer")) {
                Some(value) => Some(Manufacturer(value.as_str().expect("Manufacturer must be a string").to_string())),
                None => None
            },
            vendor: match (data.get("Vendor")) {
                Some(value) => Some(Vendor(value.as_str().expect("Vendor must be a string").to_string())),
                None => None
            },
            architecture: match (data.get("Architecture")) {
                Some(value) => Some(Architecture(value.as_str().expect("Architecture must be a string").to_string())),
                None => None
            },
            lithography: match (data.get("Lithography")) {
                Some(value) => Some(Lithography(value.as_str().expect("Lithography must be a string").to_string())),
                None => None
            },
            release_date: match (data.get("Release Date")) {
                Some(value) => Some(ReleaseDate(value.as_str().expect("Release Date must be a string").to_string())),
                None => None
            },
            tdp: match (data.get("TDP")) {
                Some(value) => Some(Tdp(value.as_str().expect("Value must be a string").to_string())),
                None => None
            },
            ray_tracing_cores: match (data.get("Ray Tracing Cores")) {
                Some(value) => Some(RayTracingCores(u16::try_from(value.as_i64().expect("Ray Tracing Cores must be i64")).expect("Ray Tracing Cores must fit into a u16"))),
                None => None
            },
            tensor_cores: match (data.get("Tensor Cores")) {
                Some(value) => Some(TensorCores(u16::try_from(value.as_i64().expect("Tensor Cores must be i64")).expect("Tensor Cores must fit into a u16"))),
                None => None
            },
        }
    }
}