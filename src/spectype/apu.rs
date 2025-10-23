use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::Serialize;
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
    boost_frequency: Option<BoostFrequency>,
    xfr_frequency: Option<XfrFrequency>,
    socket: Option<Socket>,
    stepping: Option<Stepping>,
    l1_cache_data: Option<L1CacheData>,
    l1_cache_instruction: Option<L1CacheInstruction>,
    l2_cache_total: Option<L2CacheTotal>,
    l3_cache_total: Option<L3CacheTotal>,
    memory_type: Option<MemoryType>,
    pcie_5_0_lanes: Option<Pcie50Lanes>,
    pcie_4_0_lanes: Option<Pcie40Lanes>,
    pcie_3_0_lanes: Option<Pcie30Lanes>,
    pcie_2_0_lanes: Option<Pcie20Lanes>,
    pcie_1_0_lanes: Option<Pcie10Lanes>,
    avx_sse_mmx: Option<AvxSseMmx>,
    fma4: Option<Fma4>,
    fma3: Option<Fma3>,
    bmi: Option<Bmi>,
    aes: Option<Aes>,
    sha: Option<Sha>,
    other_extensions: Option<OtherExtensions>,
    unlocked: Option<Unlocked>,
    xfr_support: Option<XfrSupport>,
    max_memory_channels: Option<MaxMemoryChannels>,
    max_memory_frequency: Option<MaxMemoryFrequency>,
    compatable_chipsets: Option<CompatableChipsets>,
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
        // Helper for Vec<String> fields
        fn get_vec_string(data: &Yaml, key: &str) -> Option<Vec<String>> {
            data[key].as_vec().map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect()
            })
        }
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
            gpu_base_frequency: match data["GPU Base Frequency"].as_str() {
                Some(value) => Some(GpuBaseFrequency(value.to_string())),
                None => None
            },
            ray_tracing_cores: match data["Ray Tracing Cores"].as_i64() {
                Some(value) => Some(RayTracingCores(u16::try_from(value).expect("Ray Tracing Cores does not fit in u16"))),
                None => None
            },
            tensor_cores: match data["Tensor Cores"].as_i64() {
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
            manufacturer: match data["Manufacturer"].as_str() {
                Some(value) => Some( Manufacturer(value.to_string())),
                None => None
            },
            architecture: match data["Architecture"].as_str() {
                Some(value) => Some(Architecture(value.to_string())),
                None => None
            },
            lithography: match data["Lithography"].as_str() {
                Some(value) => Some(Lithography(value.to_string())),
                None => None
            },
            release_date: match data["Release Date"].as_str() {
                Some(value) => Some(ReleaseDate(value.to_string())),
                None => None
            },
            tdp: match data["TDP"].as_str() {
                Some(value) => Some(Tdp(value.to_string())),
                None => None
            },
            market: get_vec_string(data, "Market").map(Market),
            boost_frequency: BoostFrequency::from_yaml(data),
            xfr_frequency: XfrFrequency::from_yaml(data),
            socket: Socket::from_yaml(data),
            stepping: Stepping::from_yaml(data),
            l1_cache_data: L1CacheData::from_yaml(data),
            l1_cache_instruction: L1CacheInstruction::from_yaml(data),
            l2_cache_total: L2CacheTotal::from_yaml(data),
            l3_cache_total: L3CacheTotal::from_yaml(data),
            memory_type: MemoryType::from_yaml(data),
            pcie_5_0_lanes: Pcie50Lanes::from_yaml(data),
            pcie_4_0_lanes: Pcie40Lanes::from_yaml(data),
            pcie_3_0_lanes: Pcie30Lanes::from_yaml(data),
            pcie_2_0_lanes: Pcie20Lanes::from_yaml(data),
            pcie_1_0_lanes: Pcie10Lanes::from_yaml(data),
            avx_sse_mmx: AvxSseMmx::from_yaml(data),
            fma4: Fma4::from_yaml(data),
            fma3: Fma3::from_yaml(data),
            bmi: Bmi::from_yaml(data),
            aes: Aes::from_yaml(data),
            sha: Sha::from_yaml(data),
            other_extensions: OtherExtensions::from_yaml(data),
            unlocked: Unlocked::from_yaml(data),
            xfr_support: XfrSupport::from_yaml(data),
            max_memory_channels: MaxMemoryChannels::from_yaml(data),
            max_memory_frequency: MaxMemoryFrequency::from_yaml(data),
            compatable_chipsets: CompatableChipsets::from_yaml(data),
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
            gpu_base_frequency: GpuBaseFrequency::from_hashmap(&data),
            direct_x_support: DirectXSupport::from_hashmap(&data),
            open_gl_support: OpenGLSupport::from_hashmap(&data),
            open_cl_support: OpenCLSupport::from_hashmap(&data),
            vulkan_support: data.get("Vulkan Support").and_then(|v| v.as_str().map(|s| VulkanSupport(s.to_string()))),
            vram_type: data.get("VRAM Type").and_then(|v| v.as_str().map(|s| VramType(s.to_string()))),
            render_output_unit_count: data.get("Render Output Unit Count").and_then(|v| v.as_i64().map(|i| RenderOutputUnitCount(u16::try_from(i).expect("Render Output Unit Count must fit into a u16")))),
            texture_mapping_unit_count: data.get("Texture Mapping Unit Count").and_then(|v| v.as_i64().map(|i| TextureMappingUnitCount(u16::try_from(i).expect("Texture Mapping Unit Count must fit into a u16")))),
            gpu_boost_frequency: data.get("GPU Boost Freqency").and_then(|v| v.as_str().map(|s| GpuBoostFrequency(s.to_string()))),
            ray_tracing_cores: RayTracingCores::from_hashmap(&data),
            tensor_cores: TensorCores::from_hashmap(&data),
            hardware_accelerated_encoding: HardwareAcceleratedEncoding::from_hashmap(&data),
            hardware_accelerated_decoding: HardwareAcceleratedDecoding::from_hashmap(&data),
            manufacturer: Manufacturer::from_hashmap(&data),
            market: Market::from_hashmap(&data),
            architecture: Architecture::from_hashmap(&data),
            lithography: Lithography::from_hashmap(&data),
            release_date: ReleaseDate::from_hashmap(&data),
            tdp: Tdp::from_hashmap(&data),
            boost_frequency: BoostFrequency::from_hashmap(&data),
            xfr_frequency: XfrFrequency::from_hashmap(&data),
            socket: Socket::from_hashmap(&data),
            stepping: Stepping::from_hashmap(&data),
            l1_cache_data: L1CacheData::from_hashmap(&data),
            l1_cache_instruction: L1CacheInstruction::from_hashmap(&data),
            l2_cache_total: L2CacheTotal::from_hashmap(&data),
            l3_cache_total: L3CacheTotal::from_hashmap(&data),
            memory_type: MemoryType::from_hashmap(&data),
            pcie_5_0_lanes: Pcie50Lanes::from_hashmap(&data),
            pcie_4_0_lanes: Pcie40Lanes::from_hashmap(&data),
            pcie_3_0_lanes: Pcie30Lanes::from_hashmap(&data),
            pcie_2_0_lanes: Pcie20Lanes::from_hashmap(&data),
            pcie_1_0_lanes: Pcie10Lanes::from_hashmap(&data),
            avx_sse_mmx: AvxSseMmx::from_hashmap(&data),
            fma4: Fma4::from_hashmap(&data),
            fma3: Fma3::from_hashmap(&data),
            bmi: Bmi::from_hashmap(&data),
            aes: Aes::from_hashmap(&data),
            sha: Sha::from_hashmap(&data),
            other_extensions: OtherExtensions::from_hashmap(&data),
            unlocked: Unlocked::from_hashmap(&data),
            xfr_support: XfrSupport::from_hashmap(&data),
            max_memory_channels: MaxMemoryChannels::from_hashmap(&data),
            max_memory_frequency: MaxMemoryFrequency::from_hashmap(&data),
            compatable_chipsets: CompatableChipsets::from_hashmap(&data),
        }
    }
}