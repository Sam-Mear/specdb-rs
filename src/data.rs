use async_graphql::{InputValueResult, Object, Scalar, ScalarType, SimpleObject, Value};
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;


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
impl_scalar_string!(ReleaseDate);
impl_scalar_vec_string!(Sockets);
impl_scalar_u16!(CoreCount);
impl_scalar_u16!(ThreadCount);
impl_scalar_string!(BaseFrequency);
impl_scalar_string!(Tdp);
impl_scalar_string!(VramCapacity);
impl_scalar_u32!(ShaderProcessorCount);
impl_scalar_string!(GpuBaseFrequency);
impl_scalar_string!(Manufacturer);
impl_scalar_string!(Vendor);
impl_scalar_string!(Architecture);
impl_scalar_u16!(TensorCores);
impl_scalar_u16!(RayTracingCores);
impl_scalar_string!(DirectXSupport);
impl_scalar_string!(OpenGLSupport);
impl_scalar_string!(OpenCLSupport);
impl_scalar_string!(VulkanSupport);
impl_scalar_vec_string!(Market);
impl_scalar_vec_string!(HardwareAcceleratedEncoding);
impl_scalar_vec_string!(HardwareAcceleratedDecoding);
impl_scalar_vec_string!(PowerConnectors);
impl_scalar_vec_string!(Outputs);
impl_scalar_string!(VramFrequency);
impl_scalar_string!(VramType);
impl_scalar_string!(VramBandwidth);
impl_scalar_string!(VramBusWidth);
impl_scalar_u16!(RenderOutputUnitCount);
impl_scalar_u16!(TextureMappingUnitCount);
impl_scalar_string!(DieSize);
impl_scalar_string!(Gpu);
impl_scalar_string!(GpuVariant);
impl_scalar_string!(HlslShaderModel);
impl_scalar_string!(GpuBoostFrequency);
impl_scalar_string!(FP32Compute);
impl_scalar_string!(FP64Compute);
impl_scalar_string!(SlotWidth);
impl_scalar_string!(Length);
impl_scalar_string!(Height);
impl_scalar_string!(Width);

