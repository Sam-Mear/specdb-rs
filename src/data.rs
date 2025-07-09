use juniper::graphql_scalar;
use serde::{Deserialize, Serialize};


#[derive(Clone)]
#[derive(Serialize)]
#[graphql_scalar]
#[graphql(
    // Custom GraphQL name.
    name = "Lithography",
    // Description can also specified in the attribute.
    // This will the doc comment, if one exists.
    description = "",
    // Optional specification URL.
    // specified_by_url = "https://tools.ietf.org/html/rfc4122",
    // Explicit generic scalar.
    scalar = S: juniper::ScalarValue,
    transparent,
)]
pub struct Lithography (pub String);

#[derive(Clone)]
#[derive(Serialize)]
#[graphql_scalar]
#[graphql(
    // Custom GraphQL name.
    name = "Release date",
    // Description can also specified in the attribute.
    // This will the doc comment, if one exists.
    description = "",
    // Optional specification URL.
    // specified_by_url = "https://tools.ietf.org/html/rfc4122",
    // Explicit generic scalar.
    scalar = S: juniper::ScalarValue,
    transparent,
)]
pub struct ReleaseDate (pub String);

#[derive(Clone)]
#[derive(Serialize)]
#[graphql_scalar]
#[graphql(
    // Custom GraphQL name.
    name = "Vector of sockets",
    // Description can also specified in the attribute.
    // This will the doc comment, if one exists.
    description = "...",
    // Optional specification URL.
    // specified_by_url = "https://tools.ietf.org/html/rfc4122",
    // Explicit generic scalar.
    scalar = V: juniper::ScalarValue,
    transparent,
)]
pub struct Sockets (pub Vec<String>);

#[derive(Clone)]
#[derive(Serialize)]
#[graphql_scalar]
#[graphql(
    with = date_scalar,
    parse_token(String),
    scalar = CustomScalarValue,
)]
pub struct CoreCount (pub u16);

#[derive(Clone)]
#[derive(Serialize)]
#[graphql_scalar]
#[graphql(
    // Custom GraphQL name.
    name = "Lithography",
    // Description can also specified in the attribute.
    // This will the doc comment, if one exists.
    description = "",
    // Optional specification URL.
    // specified_by_url = "https://tools.ietf.org/html/rfc4122",
    // Explicit generic scalar.
    scalar = V: juniper::ScalarValue,
    transparent,
)]
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
