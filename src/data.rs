use async_graphql::{InputValueResult, Object, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};


#[derive(Clone)]
#[derive(Serialize)]
pub struct Lithography (pub String);

#[Scalar]
impl ScalarType for Lithography {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(Lithography(s.to_owned()))
    } else {
        Err("Invalid value for Lithography".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct ReleaseDate (pub String);

#[Scalar]
impl ScalarType for ReleaseDate {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(ReleaseDate(s.to_owned()))
    } else {
        Err("Invalid value for ReleaseDate".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct Sockets (pub Vec<String>);

#[Scalar]
impl ScalarType for Sockets {
    fn parse(value: Value) -> InputValueResult<Self> { 
        if let Value::List(l) = &value {
            let mut sockets = Vec::new();
            for item in l {
                if let Value::String(s) = item {
                    sockets.push(s.to_owned());
                } else {
                    return Err("Invalid value in Sockets array".into());
                }
            }
            Ok(Sockets(sockets))
        } else {
            Err("Invalid value for Sockets".into())
        }
    }
    
    fn to_value(&self) -> Value { 
        let list: Vec<Value> = self.0.iter().map(|s| Value::String(s.to_owned())).collect();
        Value::List(list)
    }
}


#[derive(Clone)]
#[derive(Serialize)]
pub struct CoreCount (pub u16);

#[Scalar]
impl ScalarType for CoreCount {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::Number(n) = &value {
        if let Some(u) = n.as_u64() {
            if u <= u16::MAX as u64 {
                return Ok(CoreCount(u as u16));
            }
        }
    }
    Err("Invalid value for CoreCount".into())
  }

  fn to_value(&self) -> Value { Value::Number(self.0.into()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct ThreadCount (pub u16);

#[Scalar]
impl ScalarType for ThreadCount {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::Number(n) = &value {
        if let Some(u) = n.as_u64() {
            if u <= u16::MAX as u64 {
                return Ok(ThreadCount(u as u16));
            }
        }
    }
    Err("Invalid value for ThreadCount".into())
  }

  fn to_value(&self) -> Value { Value::Number(self.0.into()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct BaseFrequency (pub String);

#[Scalar]
impl ScalarType for BaseFrequency {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(BaseFrequency(s.to_owned()))
    } else {
        Err("Invalid value for BaseFrequency".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct Tdp (pub String);

#[Scalar]
impl ScalarType for Tdp {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(Tdp(s.to_owned()))
    } else {
        Err("Invalid value for Tdp".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct VramCapacity (pub String);

#[Scalar]
impl ScalarType for VramCapacity {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(VramCapacity(s.to_owned()))
    } else {
        Err("Invalid value for VramCapacity".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct ShaderProcessorCount (pub u32);

#[Scalar]
impl ScalarType for ShaderProcessorCount {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::Number(n) = &value {
        if let Some(u) = n.as_u64() {
            if u <= u32::MAX as u64 {
                return Ok(ShaderProcessorCount(u as u32));
            }
        }
    }
    Err("Invalid value for ShaderProcessorCount".into())
  }

  fn to_value(&self) -> Value { Value::Number(self.0.into()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct GpuBaseFrequency(pub String);

#[Scalar]
impl ScalarType for GpuBaseFrequency {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(GpuBaseFrequency(s.to_owned()))
    } else {
        Err("Invalid value for GpuBaseFrequency".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct Manufacturer(pub String);

#[Scalar]
impl ScalarType for Manufacturer {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(Manufacturer(s.to_owned()))
    } else {
        Err("Invalid value for Manufacturer".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct Vendor(pub String);

#[Scalar]
impl ScalarType for Vendor {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(Vendor(s.to_owned()))
    } else {
        Err("Invalid value for Vendor".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct Architecture(pub String);

#[Scalar]
impl ScalarType for Architecture {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::String(s) = &value {
        Ok(Architecture(s.to_owned()))
    } else {
        Err("Invalid value for Architecture".into())
    }
  }

  fn to_value(&self) -> Value { Value::String(self.0.to_owned()) }
}


#[derive(Clone)]
#[derive(Serialize)]
pub struct TensorCores (pub u16);

#[Scalar]
impl ScalarType for TensorCores {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::Number(n) = &value {
        if let Some(u) = n.as_u64() {
            if u <= u16::MAX as u64 {
                return Ok(TensorCores(u as u16));
            }
        }
    }
    Err("Invalid value for TensorCores".into())
  }

  fn to_value(&self) -> Value { Value::Number(self.0.into()) }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct RayTracingCores (pub u16);

#[Scalar]
impl ScalarType for RayTracingCores {
  fn parse(value: Value) -> InputValueResult<Self> { 
    if let Value::Number(n) = &value {
        if let Some(u) = n.as_u64() {
            if u <= u16::MAX as u64 {
                return Ok(RayTracingCores(u as u16));
            }
        }
    }
    Err("Invalid value for RayTracingCores".into())
  }

  fn to_value(&self) -> Value { Value::Number(self.0.into()) }
}