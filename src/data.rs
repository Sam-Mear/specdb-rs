use serde::{Deserialize, Serialize};


#[derive(Clone)]
#[derive(Serialize)]
pub struct Lithography {
  pub value: String
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct ReleaseDate {
  pub value: String
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct Sockets {
  pub value: Vec<String>
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct CoreCount {
  pub value: u16
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct ThreadCount {
  pub value: u16
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct BaseFrequency {
  pub value: String
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct Tdp {
  pub value: String
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct VramCapacity {
  pub value: String
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct ShaderProcessorCount {
  pub value: u32
}
#[derive(Clone)]
#[derive(Serialize)]
pub struct GpuBaseFrequency {
  pub value: String
}