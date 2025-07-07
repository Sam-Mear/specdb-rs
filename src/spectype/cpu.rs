use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
pub struct Cpu {
    core_count: CoreCount,
    thread_count: ThreadCount,
    base_frequency: BaseFrequency,
    tdp: Tdp
}
impl SpecDbType for Cpu {
    fn from_yaml(data: &Yaml) -> Self {
        let core_count = data["Core Count"].as_i64().expect("Core Count is required for type Cpu");
        let thread_count = data["Thread Count"].as_i64().expect("Thread count is required for type Cpu");
        let base_frequency = data["Base Frequency"].as_str().expect("Base Frequency is required for type Cpu").to_string();
        let tdp = data["TDP"].as_str().expect("TDP is required for type Cpu").to_string();
        Cpu {
            core_count: CoreCount { value: u16::try_from(core_count).expect("Core Count too large") },
            thread_count: ThreadCount { value: u16::try_from(thread_count).expect("Core Count too large") },
            base_frequency: BaseFrequency { value: base_frequency },
            tdp: Tdp { value: tdp }
        }
    }
    
    fn from_hashmap(data: LinkedHashMap<String, Yaml>) -> Self {
        // for (key, _) in &data {
        //     println!("key: {}", key);
        // }
        let core_count = data.get("Core Count").expect("Core Count is required for type CPU").as_i64().expect("Core Count must be int");
        let thread_count = data.get("Thread Count").expect("Thread count is required for type CPU").as_i64().expect("Thread count must be int");
        let base_frequency = data.get("Base Frequency").expect("Base Frequency is required for type CPU").as_str().expect("Base Frequency must be string").to_string();
        let tdp = data.get("TDP").expect("TDP is required for type Cpu").as_str().expect("TDP must be string").to_string();
        Cpu {
            core_count: CoreCount { value: u16::try_from(core_count).expect("Core Count too large") },
            thread_count: ThreadCount { value: u16::try_from(thread_count).expect("Core Count too large") },
            base_frequency: BaseFrequency { value: base_frequency },
            tdp: Tdp { value: tdp }
        }
    }
}