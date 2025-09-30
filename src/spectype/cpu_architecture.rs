use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct CpuArchitecture {
    lithography: Lithography,
    release_date: ReleaseDate,
    sockets: Sockets
}
impl SpecDbType for CpuArchitecture {
    fn from_yaml(data: &Yaml) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        let sockets_yaml = data["Sockets"].as_vec().expect("Sockets is required for Cpu Architecture");
        let mut sockets = Vec::new();
        for socket in sockets_yaml {
            sockets.push(socket.as_str().expect("error in socket array. Could it be coming in as an integer?").to_string());
        }
        CpuArchitecture {
            lithography: Lithography (lithography),
            release_date: ReleaseDate (release_date),
            sockets: Sockets (sockets),
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        let lithography = data["Lithography"].as_str().expect("Lithography is required for Cpu Architecture").to_string();
        let release_date = data["Release Date"].as_str().expect("Release Date is required for Cpu Architecture").to_string();
        let sockets_yaml = data["Sockets"].as_vec().expect("Sockets is required for Cpu Architecture");
        let mut sockets = Vec::new();
        for socket in sockets_yaml {
            sockets.push(socket.as_str().expect("error in socket array. Could it be coming in as an integer?").to_string());
        }
        CpuArchitecture {
            lithography: Lithography (lithography),
            release_date: ReleaseDate (release_date),
            sockets: Sockets (sockets),
        }
    }
}