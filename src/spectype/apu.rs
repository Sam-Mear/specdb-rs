use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct Apu {
    // required
    core_count: CoreCount,
    thread_count: ThreadCount,
    base_frequency: BaseFrequency,
    shader_processor_count: ShaderProcessorCount,
    // GPU specific
    gpu_base_frequency: Option<GpuBaseFrequency>,
    direct_x_support: Option<DirectXSupport>,
    open_gl_support: Option<OpenGLSupport>,
    open_cl_support: Option<OpenCLSupport>,
    vulkan_support: Option<VulkanSupport>,
    vram_type: Option<VramType>,
    render_output_unit_count: Option<RenderOutputUnitCount>,
    texture_mapping_unit_count: Option<TextureMappingUnitCount>,
    gpu_boost_frequency: Option<GpuBoostFrequency>,
    ray_tracing_cores: Option<RayTracingCores>,
    tensor_cores: Option<TensorCores>,
    hardware_accelerated_encoding: Option<HardwareAcceleratedEncoding>,
    hardware_accelerated_decoding: Option<HardwareAcceleratedDecoding>,
    // CPU Specific
    // Shared
    manufacturer: Option<Manufacturer>,
    market: Option<Market>,
    architecture: Option<Architecture>,
    lithography: Option<Lithography>,
    release_date: Option<ReleaseDate>,
    tdp: Option<Tdp>,
}
impl SpecDbType for Apu {
    fn from_yaml(data: &Yaml) -> Self {
        let core_count = data["Core Count"].as_i64().expect("Core Count is required for type Apu");
        let thread_count = data["Thread Count"].as_i64().expect("Thread count is required for type Apu");
        let base_frequency = data["Base Frequency"].as_str().expect("Base Frequency is required for type Apu").to_string();
        let shader_processor_count = data["Shader Processor Count"].as_i64().expect("Shader Processor Count is required for type Apu");
        Apu {
            core_count: CoreCount (u16::try_from(core_count).expect("Core Count too large") ),
            thread_count: ThreadCount (u16::try_from(thread_count).expect("Core Count too large") ),
            base_frequency: BaseFrequency (base_frequency ),
            shader_processor_count: ShaderProcessorCount (u32::try_from(shader_processor_count).expect("Shader processer count too high.") ),
            // GPU Specific
            gpu_base_frequency: match (data["GPU Base Frequency"].as_str()) {
                Some(value) => Some(value.to_string()),
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
            direct_x_support: match data["DirectX Support"].as_str() {
                Some(value) => Some(DirectXSupport(value.to_string())),
                None => None
            },
            open_gl_support: match data["OpenGL Support"].as_str() {
                Some(value) => Some(OpenGLSupport(value.to_string())),
                None => None
            },
            open_cl_support: match data["OpenCL Support"].as_str() {
                Some(value) => Some(OpenCLSupport(value.to_string())),
                None => None
            },
            vulkan_support: match data["Vulkan Support"].as_str() {
                Some(value) => Some(VulkanSupport(value.to_string())),
                None => None
            },
            vram_type: match data["VRAM Type"].as_str() {
                Some(value) => Some(VramType(value.to_string())),
                None => None
            },
            render_output_unit_count: match data["Render Output Unit Count"].as_i64() {
                Some(value) => Some(RenderOutputUnitCount(u16::try_from(value).expect("Render Output Unit Count must fit into a u16"))),
                None => None
            },
            texture_mapping_unit_count: match data["Texture Mapping Unit Count"].as_i64() {
                Some(value) => Some(TextureMappingUnitCount(u16::try_from(value).expect("Render Output Unit Count must fit into a u16"))),
                None => None
            },
            gpu_boost_frequency: match data["GPU Boost Freqency"].as_str() {
                Some(value) => Some(GpuBoostFrequency(value.to_string())),
                None => None
            },
            // fp32_compute: match data["FP32 Compute"].as_str() {
            //     Some(value) => Some(FP32Compute(value.to_string())),
            //     None => None
            // },
            // fp64_compute: match data["FP64 Compute"].as_str() {
            //     Some(value) => Some(FP64Compute(value.to_string())),
            //     None => None
            // },
            hardware_accelerated_encoding: get_vec_string(data, "Hardware Accelerated Encoding").map(HardwareAcceleratedEncoding),
            hardware_accelerated_decoding: get_vec_string(data, "Hardware Accelerated Decoding").map(HardwareAcceleratedDecoding),
            // Shared
            manufacturer: match (data["Manufacturer"].as_str()) {
                Some(value) => Some( Manufacturer(value.to_string())),
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
            market: get_vec_string(data, "Market").map(Market),
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
            core_count: CoreCount (u16::try_from(core_count).expect("Core Count too large") ),
            thread_count: ThreadCount (u16::try_from(thread_count).expect("Core Count too large") ),
            base_frequency: BaseFrequency (base_frequency.to_string() ),
            shader_processor_count: ShaderProcessorCount (u32::try_from(shader_processor_count).expect("Shader processer count too high.") ),
        }
    }
}