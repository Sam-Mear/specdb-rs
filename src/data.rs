use serde::{Deserialize, Serialize};


#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct Lithography {pub value: String}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct ReleaseDate {pub value: String}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct Sockets {pub value: Vec<String>}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct CoreCount {pub value: u16}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct ThreadCount {pub value: u16}



#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct BaseFrequency {pub value: String}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct Tdp {pub value: String}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct VramCapacity {pub value: String}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct ShaderProcessorCount {pub value: u32}

#[derive(Clone)]
#[derive(Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct GpuBaseFrequency{pub value: String}
