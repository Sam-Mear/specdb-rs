use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::Serialize;
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct GraphicsCard {
    pub vram_capacity: VramCapacity,
    pub shader_processor_count: ShaderProcessorCount,
    pub gpu_base_frequency: GpuBaseFrequency,
    pub manufacturer: Option<Manufacturer>,
    pub vendor: Option<Vendor>,
    pub market: Option<Market>,
    pub architecture: Option<Architecture>,
    pub lithography: Option<Lithography>,
    pub release_date: Option<ReleaseDate>,
    pub direct_x_support: Option<DirectXSupport>,
    pub open_gl_support: Option<OpenGLSupport>,
    pub open_cl_support: Option<OpenCLSupport>,
    pub vulkan_support: Option<VulkanSupport>,
    pub vram_frequency: Option<VramFrequency>,
    pub vram_type: Option<VramType>,
    pub vram_bandwidth: Option<VramBandwidth>,
    pub vram_bus_width: Option<VramBusWidth>,
    pub render_output_unit_count: Option<RenderOutputUnitCount>,
    pub texture_mapping_unit_count: Option<TextureMappingUnitCount>,
    pub die_size: Option<DieSize>,
    pub tdp: Option<Tdp>,
    pub gpu: Option<Gpu>,
    pub gpu_variant: Option<GpuVariant>,
    pub hlsl_shader_model: Option<HlslShaderModel>,
    pub gpu_boost_frequency: Option<GpuBoostFrequency>,
    pub fp32_compute: Option<FP32Compute>,
    pub fp64_compute: Option<FP64Compute>,
    pub slot_width: Option<SlotWidth>,
    pub outputs: Option<Outputs>,
    pub power_connectors: Option<PowerConnectors>,
    pub length: Option<Length>,
    pub height: Option<Height>,
    pub width: Option<Width>,
    pub ray_tracing_cores: Option<RayTracingCores>,
    pub tensor_cores: Option<TensorCores>,
    pub hardware_accelerated_encoding: Option<HardwareAcceleratedEncoding>,
    pub hardware_accelerated_decoding: Option<HardwareAcceleratedDecoding>,
    pub gpu_model: Option<GpuModel>,
    pub module_count: Option<ModuleCount>,
    pub pixel_shaders: Option<PixelShaders>,
    pub maximum_vram_capacity: Option<MaximumVramCapacity>,
    pub max_displays: Option<MaxDisplays>,
    pub crossfire_support: Option<CrossfireSupport>,
    pub free_sync_support: Option<FreeSyncSupport>,
}
impl SpecDbType for GraphicsCard {
    fn from_yaml(data: &Yaml) -> Self {
        let vram_capacity = data["VRAM Capacity"].as_str().expect("VRAM Capacity is required for type Graphics Card.").to_string();
        let shader_processor_count = data["Shader Processor Count"].as_i64().expect("Shader Processor Count is required for type Graphics Card");
        let gpu_base_frequency = data["GPU Base Frequency"].as_str().expect("GPU Base Frequency is required for type Graphics Card.").to_string();
        
        // Helper for Vec<String> fields
        fn get_vec_string(data: &Yaml, key: &str) -> Option<Vec<String>> {
            data[key].as_vec().map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect()
            })
        }
        
        GraphicsCard {
            vram_capacity: VramCapacity(vram_capacity),
            shader_processor_count: ShaderProcessorCount(u32::try_from(shader_processor_count).expect("Shader processer count too high.") ),
            gpu_base_frequency: GpuBaseFrequency(gpu_base_frequency ),
            manufacturer: match data["Manufacturer"].as_str() {
                Some(value) => Some( Manufacturer(value.to_string())),
                None => None
            },
            vendor: match data["Vendor"].as_str() {
                Some(value) => Some(Vendor(value.to_string())),
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
            ray_tracing_cores: match data["Ray Tracing Cores"].as_i64() {
                Some(value) => Some(RayTracingCores(u16::try_from(value).expect("Ray Tracing Cores does not fit in u16"))),
                None => None
            },
            tensor_cores: match data["Tensor Cores"].as_i64() {
                Some(value) => Some(TensorCores(u16::try_from(value).expect("Tensor Cores does not fit into u16"))),
                None => None
            },
            market: get_vec_string(data, "Market").map(Market),
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
            vram_frequency: match data["VRAM Frequency"].as_str() {
                Some(value) => Some(VramFrequency(value.to_string())),
                None => None
            },
            vram_type: match data["VRAM Type"].as_str() {
                Some(value) => Some(VramType(value.to_string())),
                None => None
            },
            vram_bandwidth: match data["VRAM Bandwidth"].as_str() {
                Some(value) => Some(VramBandwidth(value.to_string())),
                None => None
            },
            vram_bus_width: match data["VRAM Bus Width"].as_str() {
                Some(value) => Some(VramBusWidth(value.to_string())),
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
            die_size: match data["Die Size"].as_str() {
                Some(value) => Some(DieSize(value.to_string())),
                None => None
            },
            gpu: match data["GPU"].as_str() {
                Some(value) => Some(Gpu(value.to_string())),
                None => None
            },
            gpu_variant: match data["GPU Variant"].as_str() {
                Some(value) => Some(GpuVariant(value.to_string())),
                None => None
            },
            hlsl_shader_model: match data["HLSL Shader Model"].as_str() {
                Some(value) => Some(HlslShaderModel(value.to_string())),
                None => None
            },
            gpu_boost_frequency: match data["GPU Boost Freqency"].as_str() {
                Some(value) => Some(GpuBoostFrequency(value.to_string())),
                None => None
            },
            fp32_compute: match data["FP32 Compute"].as_str() {
                Some(value) => Some(FP32Compute(value.to_string())),
                None => None
            },
            fp64_compute: match data["FP64 Compute"].as_str() {
                Some(value) => Some(FP64Compute(value.to_string())),
                None => None
            },
            slot_width: match data["Slot Width"].as_str() {
                Some(value) => Some(SlotWidth(value.to_string())),
                None => None
            },
            outputs: get_vec_string(data, "Outputs").map(Outputs),
            power_connectors: get_vec_string(data, "Power Connectors").map(PowerConnectors),
            length: match data["Length"].as_str() {
                Some(value) => Some(Length(value.to_string())),
                None => None
            },
            height: match data["Height"].as_str() {
                Some(value) => Some(Height(value.to_string())),
                None => None
            },
            width: match data["Width"].as_str() {
                Some(value) => Some(Width(value.to_string())),
                None => None
            },
            hardware_accelerated_encoding: get_vec_string(data, "Hardware Accelerated Encoding").map(HardwareAcceleratedEncoding),
            hardware_accelerated_decoding: get_vec_string(data, "Hardware Accelerated Decoding").map(HardwareAcceleratedDecoding),
            gpu_model: GpuModel::from_yaml(data),
            module_count: ModuleCount::from_yaml(data),
            pixel_shaders: PixelShaders::from_yaml(data),
            maximum_vram_capacity: MaximumVramCapacity::from_yaml(data),
            max_displays: MaxDisplays::from_yaml(data),
            crossfire_support: CrossfireSupport::from_yaml(data),
            free_sync_support: FreeSyncSupport::from_yaml(data),
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

        let get_vec_string = |key: &str| -> Option<Vec<String>> {
            data.get(key).and_then(|v| {
                if let Some(arr) = v.as_vec() {
                    Some(arr.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect())
                } else {
                    None
                }
            })
        };

        GraphicsCard {
            vram_capacity: VramCapacity (vram_capacity.to_string()),
            shader_processor_count: ShaderProcessorCount(u32::try_from(shader_processor_count).expect("Shader processer count too high.")),
            gpu_base_frequency: GpuBaseFrequency (gpu_base_frequency),
            manufacturer: match data.get("Manufacturer") {
                Some(value) => Some(Manufacturer(value.as_str().expect("Manufacturer must be a string").to_string())),
                None => None
            },
            vendor: match data.get("Vendor") {
                Some(value) => Some(Vendor(value.as_str().expect("Vendor must be a string").to_string())),
                None => None
            },
            architecture: match data.get("Architecture") {
                Some(value) => Some(Architecture(value.as_str().expect("Architecture must be a string").to_string())),
                None => None
            },
            lithography: match data.get("Lithography") {
                Some(value) => Some(Lithography(value.as_str().expect("Lithography must be a string").to_string())),
                None => None
            },
            release_date: match data.get("Release Date") {
                Some(value) => Some(ReleaseDate(value.as_str().expect("Release Date must be a string").to_string())),
                None => None
            },
            tdp: match data.get("TDP") {
                Some(value) => Some(Tdp(value.as_str().expect("Value must be a string").to_string())),
                None => None
            },
            ray_tracing_cores: match data.get("Ray Tracing Cores") {
                Some(value) => Some(RayTracingCores(u16::try_from(value.as_i64().expect("Ray Tracing Cores must be i64")).expect("Ray Tracing Cores must fit into a u16"))),
                None => None
            },
            tensor_cores: match data.get("Tensor Cores") {
                Some(value) => Some(TensorCores(u16::try_from(value.as_i64().expect("Tensor Cores must be i64")).expect("Tensor Cores must fit into a u16"))),
                None => None
            },
            market: get_vec_string("Market").map(Market),
            direct_x_support: data.get("DirectX Support").and_then(|v| v.as_str().map(|s| DirectXSupport(s.to_string()))),
            open_gl_support: data.get("OpenGL Support").and_then(|v| v.as_str().map(|s| OpenGLSupport(s.to_string()))),
            open_cl_support: data.get("OpenCL Support").and_then(|v| v.as_str().map(|s| OpenCLSupport(s.to_string()))),
            vulkan_support: data.get("Vulkan Support").and_then(|v| v.as_str().map(|s| VulkanSupport(s.to_string()))),
            vram_frequency: data.get("VRAM Frequency").and_then(|v| v.as_str().map(|s| VramFrequency(s.to_string()))),
            vram_type: data.get("VRAM Type").and_then(|v| v.as_str().map(|s| VramType(s.to_string()))),
            vram_bandwidth: data.get("VRAM Bandwidth").and_then(|v| v.as_str().map(|s| VramBandwidth(s.to_string()))),
            vram_bus_width: data.get("VRAM Bus Width").and_then(|v| v.as_str().map(|s| VramBusWidth(s.to_string()))),
            render_output_unit_count: data.get("Render Output Unit Count").and_then(|v| v.as_i64().map(|i| RenderOutputUnitCount(u16::try_from(i).expect("Render Output Unit Count must fit into a u16")))),
            texture_mapping_unit_count: data.get("Texture Mapping Unit Count").and_then(|v| v.as_i64().map(|i| TextureMappingUnitCount(u16::try_from(i).expect("Texture Mapping Unit Count must fit into a u16")))),
            die_size: data.get("Die Size").and_then(|v| v.as_str().map(|s| DieSize(s.to_string()))),
            gpu: data.get("GPU").and_then(|v| v.as_str().map(|s| Gpu(s.to_string()))),
            gpu_variant: data.get("GPU Variant").and_then(|v| v.as_str().map(|s| GpuVariant(s.to_string()))),
            hlsl_shader_model: data.get("HLSL Shader Model").and_then(|v| v.as_str().map(|s| HlslShaderModel(s.to_string()))),
            gpu_boost_frequency: data.get("GPU Boost Freqency").and_then(|v| v.as_str().map(|s| GpuBoostFrequency(s.to_string()))),
            fp32_compute: data.get("FP32 Compute").and_then(|v| v.as_str().map(|s| FP32Compute(s.to_string()))),
            fp64_compute: data.get("FP64 Compute").and_then(|v| v.as_str().map(|s| FP64Compute(s.to_string()))),
            slot_width: data.get("Slot Width").and_then(|v| v.as_str().map(|s| SlotWidth(s.to_string()))),
            outputs: get_vec_string("Outputs").map(Outputs),
            power_connectors: get_vec_string("Power Connectors").map(PowerConnectors),
            length: data.get("Length").and_then(|v| v.as_str().map(|s| Length(s.to_string()))),
            height: data.get("Height").and_then(|v| v.as_str().map(|s| Height(s.to_string()))),
            width: data.get("Width").and_then(|v| v.as_str().map(|s| Width(s.to_string()))),
            hardware_accelerated_encoding: get_vec_string("Hardware Accelerated Encoding").map(HardwareAcceleratedEncoding),
            hardware_accelerated_decoding: get_vec_string("Hardware Accelerated Decoding").map(HardwareAcceleratedDecoding),
            gpu_model: GpuModel::from_hashmap(&data),
            module_count: ModuleCount::from_hashmap(&data),
            pixel_shaders: PixelShaders::from_hashmap(&data),
            maximum_vram_capacity: MaximumVramCapacity::from_hashmap(&data),
            max_displays: MaxDisplays::from_hashmap(&data),
            crossfire_support: CrossfireSupport::from_hashmap(&data),
            free_sync_support: FreeSyncSupport::from_hashmap(&data),
        }
    }
}