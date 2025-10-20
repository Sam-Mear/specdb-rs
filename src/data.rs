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

#[derive(Clone)]
#[derive(Serialize)]
pub struct Tdp (pub String);

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
                return data.get("$yaml_label").and_then(|v| v.as_i64().map(|num| {
                    $name(u32::try_from(num).expect("Value too large"))
                }));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data["$yaml_label"].as_i64() {
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
                return data.get("$yaml_label").and_then(|v| v.as_i64().map(|num| {
                    $name(u16::try_from(num).expect("Value too large"))
                }));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data["$yaml_label"].as_i64() {
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
                return data.get("$yaml_label").and_then(|v| v.as_vec().map(|arr| {
                    let items: Vec<String> = arr.iter()
                        .filter_map(|item| item.as_str().map(|s| s.to_string()))
                        .collect();
                    $name(items)
                }));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data["$yaml_label"].as_vec() {
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
                return data.get("$yaml_label").and_then(|v| v.as_str().map(|s| $name(s.to_string())));
            }
            pub fn from_yaml(data: &Yaml) -> Option<Self> {
                match data["$yaml_label"].as_str() {
                    Some(value) => Some($name(value.to_string())),
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

