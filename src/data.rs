use async_graphql::{InputValueResult, Scalar, ScalarType, SimpleObject, Value};
use serde::Serialize;
use yaml_rust2::Yaml;
use hashlink::LinkedHashMap;


#[derive(Clone)]
#[derive(Serialize)]
pub struct Lithography (pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct ReleaseDate (pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct Sockets (pub Vec<String>);


#[derive(Clone)]
#[derive(Serialize)]
pub struct CoreCount (pub u16);

#[derive(Clone)]
#[derive(Serialize)]
pub struct ThreadCount (pub u16);

#[derive(Clone)]
#[derive(Serialize)]
pub struct BaseFrequency (pub String);

#[derive(Clone, Serialize)]
pub struct PerformanceCoreBaseFrequency (pub String);

#[derive(Clone, Serialize)]
pub struct EfficientCoreBaseFrequency (pub String);

#[derive(Clone, Serialize)]
pub struct PerformanceCoreBoostFrequency (pub String);

#[derive(Clone, Serialize)]
pub struct EfficientCoreBoostFrequency (pub String);

#[derive(Clone, Serialize)]
pub struct PerformanceCoreCount (pub u16);

#[derive(Clone, Serialize)]
pub struct EfficientCoreCount (pub u16);

#[derive(Clone, Serialize)]
pub struct PerformanceThreadCount (pub u16);

#[derive(Clone, Serialize)]
pub struct EfficientThreadCount (pub u16);

#[derive(Clone)]
#[derive(Serialize)]
pub struct Tdp (pub String);

#[derive(Clone, Serialize)]
pub struct CtdpSupport (pub bool);

#[derive(Clone)]
#[derive(Serialize)]
pub struct VramCapacity (pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct ShaderProcessorCount (pub u32);

#[derive(Clone)]
#[derive(Serialize)]
pub struct GpuBaseFrequency(pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct Manufacturer(pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct Vendor(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct Architecture(pub String);

#[derive(Clone, Serialize)]
pub struct EfficientCoreArchitecture(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct TensorCores (pub u16);

#[derive(Clone)]
#[derive(Serialize)]
pub struct RayTracingCores (pub u16);

// These suppots are either int.int or 'N/A'
// todo: account for this
#[derive(Clone)]
#[derive(Serialize)]
pub struct DirectXSupport(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct OpenGLSupport(pub String);



#[derive(Clone)]
#[derive(Serialize)]
pub struct OpenCLSupport(pub String);



#[derive(Clone)]
#[derive(Serialize)]
pub struct VulkanSupport(pub String);



#[derive(Clone)]
#[derive(Serialize)]
pub struct Market(pub Vec<String>);

#[derive(Clone)]
#[derive(Serialize)]
pub struct HardwareAcceleratedEncoding(pub Vec<String>);



#[derive(Clone)]
#[derive(Serialize)]
pub struct HardwareAcceleratedDecoding(pub Vec<String>);


#[derive(Clone)]
#[derive(Serialize)]
pub struct PowerConnectors(pub Vec<String>);


#[derive(Clone)]
#[derive(Serialize)]
pub struct Outputs(pub Vec<String>);


#[derive(Clone)]
#[derive(Serialize)]
pub struct VramFrequency(pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct VramType(pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct VramBandwidth(pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct VramBusWidth(pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct RenderOutputUnitCount(pub u16);

#[derive(Clone)]
#[derive(Serialize)]
pub struct TextureMappingUnitCount(pub u16);

#[derive(Clone)]
#[derive(Serialize)]
pub struct DieSize(pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct Gpu(pub String);

#[derive(Clone, Serialize)]
pub struct GpuModel(pub String);

#[derive(Clone)]
#[derive(Serialize)]
pub struct GpuVariant(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct HlslShaderModel(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct GpuBoostFrequency(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct FP32Compute(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct FP64Compute(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct SlotWidth(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct Length(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct Height(pub String);


#[derive(Clone)]
#[derive(Serialize)]
pub struct Width(pub String);

#[derive(Clone, Serialize)]
pub struct BoostFrequency(pub String);

#[derive(Clone, Serialize)]
pub struct XfrFrequency(pub String);

#[derive(Clone, Serialize)]
pub struct Socket(pub String);


#[derive(Clone, Serialize)]
pub struct Stepping(pub String);

#[derive(Clone, Serialize)]
pub struct L1CacheData(pub String);


#[derive(Clone, Serialize)]
pub struct L1CacheInstruction(pub String);


#[derive(Clone, Serialize)]
pub struct L2CacheTotal(pub String);

#[derive(Clone, Serialize)]
pub struct L3CacheTotal(pub String);

#[derive(Clone, Serialize)]
pub struct MemoryType(pub String);

#[derive(Clone, Serialize)]
pub struct Pcie10Lanes(pub u32);

#[derive(Clone, Serialize)]
pub struct Pcie20Lanes(pub u32);

#[derive(Clone, Serialize)]
pub struct Pcie30Lanes(pub u32);

#[derive(Clone, Serialize)]
pub struct Pcie40Lanes(pub u32);

#[derive(Clone, Serialize)]
pub struct Pcie50Lanes(pub u32);

#[derive(Clone, Serialize)]
pub struct AvxSseMmx(pub String);

#[derive(Clone, Serialize)]
pub struct Fma4(pub bool);

#[derive(Clone, Serialize)]
pub struct Fma3(pub bool);

#[derive(Clone, Serialize)]
pub struct Bmi(pub String);

#[derive(Clone, Serialize)]
pub struct Aes(pub bool);

#[derive(Clone, Serialize)]
pub struct Sha(pub bool);

#[derive(Clone, Serialize)]
pub struct OtherExtensions(pub Vec<String>);

#[derive(Clone, Serialize)]
pub struct Unlocked(pub bool);

#[derive(Clone, Serialize)]
pub struct XfrSupport(pub bool);

#[derive(Clone, Serialize)]
pub struct MaxMemoryFrequency(pub String);

#[derive(Clone, Serialize)]
pub struct MaxMemoryChannels(pub String);

#[derive(Clone, Serialize)]
pub struct CompatableChipsets(pub Vec<String>);

#[derive(Clone, Serialize)]
pub struct ModuleCount(pub String);

#[derive(Clone, Serialize)]
pub struct PixelShaders(pub u16);

#[derive(Clone, Serialize)]
pub struct MaximumVramCapacity(pub String);

#[derive(Clone, Serialize)]
pub struct MaxDisplays(pub String);

#[derive(Clone, Serialize)]
pub struct CrossfireSupport(pub String);

#[derive(Clone, Serialize)]
pub struct FreeSyncSupport(pub String);


pub enum SectionError {
    TypeError { message: String , required: bool},
}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct Section {
    pub header: String,
    pub members: Vec<String>
}

impl Section {
    pub fn from_yaml(data: &Yaml) -> Result<Vec<Self>, SectionError> {
        // match data["sections"].as_vec() {
        //     Some(vector) => {
        //         let mut result = Vec::<Self>::new();
        //         for section in vector.into_iter(){
        //             result.push(Section {
        //                 header: match section["header"].as_str() {
        //                     Some(value) => return value.to_string(),
        //                     None => ,
        //                 },
        //                 members: () 
        //             })
        //         }
        //     },
        //     None => return Err("Sections is not an array."),
        // }
        
        let sections = match data["sections"].as_vec() {
            Some(vector) => {
                vector
            },
            None => return Err(SectionError::TypeError{message: "Sections is not an array.".to_string(), required: false}),
        };
        let mut result = Vec::<Self>::new();
        for section in sections.into_iter(){
            let header = match section["header"].as_str() {
                    Some(value) => value.to_string(),
                    None => return Err(SectionError::TypeError { message: "Header is not a string".to_string(), required: true }),
            };
            let members_yaml = match section["members"].as_vec() {
                Some(value) => value,
                None => return Err(SectionError::TypeError { message: "Members is not an array for NAMEHERETODO".to_string(), required: true })
            };
            let mut members = Vec::<String>::new();
            for member in members_yaml.into_iter() {
                members.push(match member.as_str() {
                    Some(value) => value.to_string(),
                    None => return Err(SectionError::TypeError { message: "Member is not a string".to_string(), required: true })
                })
            }
            result.push(Section {
                header: header,
                members: members
            })
        }
        return Ok(result);
    }
}

macro_rules! impl_from_parser_u32 {
    ($name:ident, $yaml_label:expr) => {
        impl $name {
            pub fn from_hashmap(data: &LinkedHashMap<String, Yaml>) -> Option<Self> {
                return data.get($yaml_label).and_then(|v| v.as_i64().map(|num| {
                    $name(u32::try_from(num).expect("Value too large"))
                }));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data[$yaml_label].as_i64() {
                    Some(value) => Some($name(u32::try_from(value).expect("Value too large"))),
                    None => None
                }
            }
        }
    };
}

macro_rules! impl_from_parser_u16 {
    ($name:ident, $yaml_label:expr) => {
        impl $name {
            pub fn from_hashmap(data: &LinkedHashMap<String, Yaml>) -> Option<Self> {
                return data.get($yaml_label).and_then(|v| v.as_i64().map(|num| {
                    $name(u16::try_from(num).expect("Value too large"))
                }));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data[$yaml_label].as_i64() {
                    Some(value) => Some($name(u16::try_from(value).expect("Value too large"))),
                    None => None
                }
            }
        }
    };
}

macro_rules! impl_from_parser_vec_string {
    ($name:ident, $yaml_label:expr) => {
        impl $name {
            pub fn from_hashmap(data: &LinkedHashMap<String, Yaml>) -> Option<Self> {
                return data.get($yaml_label).and_then(|v| v.as_vec().map(|arr| {
                    let items: Vec<String> = arr.iter()
                        .filter_map(|item| item.as_str().map(|s| s.to_string()))
                        .collect();
                    $name(items)
                }));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data[$yaml_label].as_vec() {
                    Some(arr) => {
                        let items: Vec<String> = arr.iter()
                            .filter_map(|item| item.as_str().map(|s| s.to_string()))
                            .collect();
                        Some($name(items))
                    },
                    None => None
                }
            }
        }
    };
}

macro_rules! impl_from_parser_string {
    ($name:ident, $yaml_label:expr) => {
        impl $name {
            pub fn from_hashmap(data: &LinkedHashMap<String, Yaml>) -> Option<Self> {
                return data.get($yaml_label).and_then(|v| v.as_str().map(|s| $name(s.to_string())));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data[$yaml_label].as_str() {
                    Some(value) => Some($name(value.to_string())),
                    None => None
                }
            }
        }
    };
}

macro_rules! impl_from_parser_bool {
    ($name:ident, $yaml_label:expr) => {
        impl $name {
            pub fn from_hashmap(data: &LinkedHashMap<String, Yaml>) -> Option<Self> {
                return data.get($yaml_label).and_then(|v| v.as_bool().map(|b| $name(b)));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data[$yaml_label].as_bool() {
                    Some(value) => Some($name(value)),
                    None => None
                }
            }
        }
    };
}

macro_rules! impl_scalar_string {
    ($name:ident) => {
        #[Scalar]
        impl ScalarType for $name {
            fn parse(value: Value) -> InputValueResult<Self> {
                if let Value::String(s) = &value {
                    Ok($name(s.to_owned()))
                } else {
                    Err(format!("Invalid value for {}", stringify!($name)).into())
                }
            }
            fn to_value(&self) -> Value {
                Value::String(self.0.to_owned())
            }
        }
    };
}

macro_rules! impl_scalar_bool {
    ($name:ident) => {
        #[Scalar]
        impl ScalarType for $name {
            fn parse(value: Value) -> InputValueResult<Self> {
                if let Value::Boolean(b) = &value {
                    Ok($name(*b))
                } else {
                    Err(format!("Invalid value for {}", stringify!($name)).into())
                }
            }
            fn to_value(&self) -> Value {
                Value::Boolean(self.0)
            }
        }
    };
}

macro_rules! impl_scalar_u16 {
    ($name:ident) => {
        #[Scalar]
        impl ScalarType for $name {
            fn parse(value: Value) -> InputValueResult<Self> {
                if let Value::Number(n) = &value {
                    if let Some(u) = n.as_u64() {
                        if u <= u16::MAX as u64 {
                            return Ok($name(u as u16));
                        }
                    }
                }
                Err(format!("Invalid value for {}", stringify!($name)).into())
            }
            fn to_value(&self) -> Value {
                Value::Number(self.0.into())
            }
        }
    };
}

macro_rules! impl_scalar_u32 {
    ($name:ident) => {
        #[Scalar]
        impl ScalarType for $name {
            fn parse(value: Value) -> InputValueResult<Self> {
                if let Value::Number(n) = &value {
                    if let Some(u) = n.as_u64() {
                        if u <= u32::MAX as u64 {
                            return Ok($name(u as u32));
                        }
                    }
                }
                Err(format!("Invalid value for {}", stringify!($name)).into())
            }
            fn to_value(&self) -> Value {
                Value::Number(self.0.into())
            }
        }
    };
}

macro_rules! impl_scalar_vec_string {
    ($name:ident) => {
        #[Scalar]
        impl ScalarType for $name {
            fn parse(value: Value) -> InputValueResult<Self> {
                if let Value::List(l) = &value {
                    let mut items = Vec::new();
                    for item in l {
                        if let Value::String(s) = item {
                            items.push(s.to_owned());
                        } else {
                            return Err(format!("Invalid value in {} array", stringify!($name)).into());
                        }
                    }
                    Ok($name(items))
                } else {
                    Err(format!("Invalid value for {}", stringify!($name)).into())
                }
            }
            fn to_value(&self) -> Value {
                let list: Vec<Value> = self.0.iter().map(|s| Value::String(s.to_owned())).collect();
                Value::List(list)
            }
        }
    };
}


impl_scalar_string!(Lithography);
impl_from_parser_string!(Lithography, "Lithography");
impl_scalar_string!(ReleaseDate);
impl_from_parser_string!(ReleaseDate, "Release Date");
impl_scalar_vec_string!(Sockets);
impl_from_parser_vec_string!(Sockets, "Sockets");
impl_scalar_u16!(CoreCount);
impl_from_parser_u16!(CoreCount, "Core Count");
impl_scalar_u16!(ThreadCount);
impl_from_parser_u16!(ThreadCount, "Thread Count");
impl_scalar_string!(BaseFrequency);
impl_from_parser_string!(BaseFrequency, "Base Frequency");
impl_scalar_string!(Tdp);
impl_from_parser_string!(Tdp, "TDP");
impl_scalar_string!(VramCapacity);
impl_from_parser_string!(VramCapacity, "VRAM Capacity");
impl_scalar_u32!(ShaderProcessorCount);
impl_from_parser_u32!(ShaderProcessorCount, "Shader Processor Count");
impl_scalar_string!(GpuBaseFrequency);
impl_from_parser_string!(GpuBaseFrequency, "GPU Base Frequency");
impl_scalar_string!(Manufacturer);
impl_from_parser_string!(Manufacturer, "Manufacturer");
impl_scalar_string!(Vendor);
impl_from_parser_string!(Vendor, "Vendor");
impl_scalar_string!(Architecture);
impl_from_parser_string!(Architecture, "Architecture");
impl_scalar_u16!(TensorCores);
impl_from_parser_u16!(TensorCores, "Tensor Cores");
impl_scalar_u16!(RayTracingCores);
impl_from_parser_u16!(RayTracingCores, "Ray Tracing Cores");
impl_scalar_string!(DirectXSupport);
impl_from_parser_string!(DirectXSupport, "DirectX Support");
impl_scalar_string!(OpenGLSupport);
impl_from_parser_string!(OpenGLSupport, "OpenGL Support");
impl_scalar_string!(OpenCLSupport);
impl_from_parser_string!(OpenCLSupport, "OpenCL Support");
impl_scalar_string!(VulkanSupport);
impl_from_parser_string!(VulkanSupport, "Vulkan Support");
impl_scalar_vec_string!(Market);
impl_from_parser_vec_string!(Market, "Market");
impl_scalar_vec_string!(HardwareAcceleratedEncoding);
impl_from_parser_vec_string!(HardwareAcceleratedEncoding, "Hardware Accelerated Encoding");
impl_scalar_vec_string!(HardwareAcceleratedDecoding);
impl_from_parser_vec_string!(HardwareAcceleratedDecoding, "Hardware Accelerated Decoding");
impl_scalar_vec_string!(PowerConnectors);
impl_from_parser_vec_string!(PowerConnectors, "Power Connectors");
impl_scalar_vec_string!(Outputs);
impl_from_parser_vec_string!(Outputs, "Outputs");
impl_scalar_string!(VramFrequency);
impl_from_parser_string!(VramFrequency, "VRAM Frequency");
impl_scalar_string!(VramType);
impl_from_parser_string!(VramType, "VRAM Type");
impl_scalar_string!(VramBandwidth);
impl_from_parser_string!(VramBandwidth, "VRAM Bandwidth");
impl_scalar_string!(VramBusWidth);
impl_from_parser_string!(VramBusWidth, "VRAM Bus Width");
impl_scalar_u16!(RenderOutputUnitCount);
impl_from_parser_u16!(RenderOutputUnitCount, "Render Output Unit Count");
impl_scalar_u16!(TextureMappingUnitCount);
impl_from_parser_u16!(TextureMappingUnitCount, "Texture Mapping Unit Count");
impl_scalar_string!(DieSize);
impl_from_parser_string!(DieSize, "Die Size");
impl_scalar_string!(Gpu);
impl_from_parser_string!(Gpu, "GPU");
impl_scalar_string!(GpuVariant);
impl_from_parser_string!(GpuVariant, "GPU Variant");
impl_scalar_string!(HlslShaderModel);
impl_from_parser_string!(HlslShaderModel, "HLSL Shader Model");
impl_scalar_string!(GpuBoostFrequency);
impl_from_parser_string!(GpuBoostFrequency, "GPU Boost Frequency");
impl_scalar_string!(FP32Compute);
impl_from_parser_string!(FP32Compute, "FP32 Compute");
impl_scalar_string!(FP64Compute);
impl_from_parser_string!(FP64Compute, "FP64 Compute");
impl_scalar_string!(SlotWidth);
impl_from_parser_string!(SlotWidth, "Slot Width");
impl_scalar_string!(Length);
impl_from_parser_string!(Length, "Length");
impl_scalar_string!(Height);
impl_from_parser_string!(Height, "Height");
impl_scalar_string!(Width);
impl_from_parser_string!(Width, "Width");
impl_scalar_string!(BoostFrequency);
impl_from_parser_string!(BoostFrequency, "Boost Frequency");
impl_scalar_string!(XfrFrequency);
impl_from_parser_string!(XfrFrequency, "XFR Frequency");
impl_scalar_string!(Socket);
impl_from_parser_string!(Socket, "Socket");
impl_scalar_string!(Stepping);
impl_from_parser_string!(Stepping, "Stepping");
impl_scalar_string!(L1CacheData);
impl_from_parser_string!(L1CacheData, "L1 Cache (Data)");
impl_scalar_string!(L1CacheInstruction);
impl_from_parser_string!(L1CacheInstruction, "L1 Cache (Instruction)");
impl_scalar_string!(L2CacheTotal);
impl_from_parser_string!(L2CacheTotal, "L2 Cache (Total)");
impl_scalar_string!(L3CacheTotal);
impl_from_parser_string!(L3CacheTotal, "L3 Cache (Total)");
impl_scalar_string!(MemoryType);
impl_from_parser_string!(MemoryType, "Memory Type");
impl_scalar_u32!(Pcie10Lanes);
impl_from_parser_u32!(Pcie10Lanes, "PCIe 1.0 Lanes");
impl_scalar_u32!(Pcie20Lanes);
impl_from_parser_u32!(Pcie20Lanes, "PCIe 2.0 Lanes");
impl_scalar_u32!(Pcie30Lanes);
impl_from_parser_u32!(Pcie30Lanes, "PCIe 3.0 Lanes");
impl_scalar_u32!(Pcie40Lanes);
impl_from_parser_u32!(Pcie40Lanes, "PCIe 4.0 Lanes");
impl_scalar_u32!(Pcie50Lanes);
impl_from_parser_u32!(Pcie50Lanes, "PCIe 5.0 Lanes");
impl_scalar_string!(AvxSseMmx);
impl_from_parser_string!(AvxSseMmx, "AVX/SSE/MMX");
impl_scalar_bool!(Fma4);
impl_from_parser_bool!(Fma4, "FMA4");
impl_scalar_bool!(Fma3);
impl_from_parser_bool!(Fma3, "FMA3");
impl_scalar_string!(Bmi);
impl_from_parser_string!(Bmi, "BMI");
impl_scalar_bool!(Aes);
impl_from_parser_bool!(Aes, "AES");
impl_scalar_bool!(Sha);
impl_from_parser_bool!(Sha, "SHA");
impl_scalar_vec_string!(OtherExtensions);
impl_from_parser_vec_string!(OtherExtensions, "Other Extensions");
impl_scalar_bool!(Unlocked);
impl_from_parser_bool!(Unlocked, "Unlocked");
impl_scalar_bool!(XfrSupport);
impl_from_parser_bool!(XfrSupport, "XFR Support");
impl_scalar_string!(MaxMemoryFrequency);
impl_from_parser_string!(MaxMemoryFrequency, "Max Memory Frequency");
impl_scalar_string!(MaxMemoryChannels);
impl_from_parser_string!(MaxMemoryChannels, "Max Memory Channels");
impl_scalar_vec_string!(CompatableChipsets);
impl_from_parser_vec_string!(CompatableChipsets, "Compatable Chipsets");
impl_scalar_string!(PerformanceCoreBaseFrequency);
impl_from_parser_string!(PerformanceCoreBaseFrequency,"Performance-Core Base Frequency");
impl_scalar_string!(EfficientCoreBaseFrequency);
impl_from_parser_string!(EfficientCoreBaseFrequency,"Efficient-Core Base Frequency");
impl_scalar_string!(PerformanceCoreBoostFrequency);
impl_from_parser_string!(PerformanceCoreBoostFrequency,"Performance-Core Boost Frequency");
impl_scalar_string!(EfficientCoreBoostFrequency);
impl_from_parser_string!(EfficientCoreBoostFrequency,"Efficient-Core Boost Frequency");
impl_scalar_string!(EfficientCoreArchitecture);
impl_from_parser_string!(EfficientCoreArchitecture,"Efficient-Core Architecture");
impl_scalar_string!(GpuModel);
impl_from_parser_string!(GpuModel,"Gpu Model");
impl_scalar_string!(ModuleCount);
impl_from_parser_string!(ModuleCount,"Module Count");
impl_scalar_string!(MaximumVramCapacity);
impl_from_parser_string!(MaximumVramCapacity,"Maximum VRAM Capacity");
impl_scalar_string!(MaxDisplays);
impl_from_parser_string!(MaxDisplays,"Max Displays");
impl_scalar_string!(CrossfireSupport);
impl_from_parser_string!(CrossfireSupport,"Crossfire Support");
impl_scalar_string!(FreeSyncSupport);
impl_from_parser_string!(FreeSyncSupport,"FreeSync Support");
impl_scalar_u16!(PerformanceCoreCount);
impl_from_parser_u16!(PerformanceCoreCount, "Performance-Core Count");
impl_scalar_u16!(EfficientCoreCount);
impl_from_parser_u16!(EfficientCoreCount, "Efficient-Core Count");
impl_scalar_u16!(PerformanceThreadCount);
impl_from_parser_u16!(PerformanceThreadCount, "Performance-Thread Count");
impl_scalar_u16!(EfficientThreadCount);
impl_from_parser_u16!(EfficientThreadCount, "Efficient-Thread Count");
impl_scalar_u16!(PixelShaders);
impl_from_parser_u16!(PixelShaders, "Pixel Shaders");
impl_scalar_bool!(CtdpSupport);
impl_from_parser_bool!(CtdpSupport, "cTDP Support");
