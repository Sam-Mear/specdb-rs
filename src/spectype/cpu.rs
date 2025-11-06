use async_graphql::SimpleObject;
use hashlink::LinkedHashMap;
use serde::{Serialize};
use yaml_rust2::Yaml;

use crate::{data::*, spectype::SpecDbType};

#[derive(Clone)]
#[derive(Serialize)]
#[derive(SimpleObject)]
pub struct Cpu {
    pub core_count: CoreCount,
    pub thread_count: ThreadCount,
    pub base_frequency: BaseFrequency,
    pub tdp: Tdp,
    // CPU Specific
    pub boost_frequency: Option<BoostFrequency>,
    pub xfr_frequency: Option<XfrFrequency>,
    pub socket: Option<Socket>,
    pub stepping: Option<Stepping>,
    pub l1_cache_data: Option<L1CacheData>,
    pub l1_cache_instruction: Option<L1CacheInstruction>,
    pub l2_cache_total: Option<L2CacheTotal>,
    pub l3_cache_total: Option<L3CacheTotal>,
    pub memory_type: Option<MemoryType>,
    pub pcie_5_0_lanes: Option<Pcie50Lanes>,
    pub pcie_4_0_lanes: Option<Pcie40Lanes>,
    pub pcie_3_0_lanes: Option<Pcie30Lanes>,
    pub pcie_2_0_lanes: Option<Pcie20Lanes>,
    pub pcie_1_0_lanes: Option<Pcie10Lanes>,
    pub avx_sse_mmx: Option<AvxSseMmx>,
    pub fma4: Option<Fma4>,
    pub fma3: Option<Fma3>,
    pub bmi: Option<Bmi>,
    pub aes: Option<Aes>,
    pub sha: Option<Sha>,
    pub other_extensions: Option<OtherExtensions>,
    pub unlocked: Option<Unlocked>,
    pub xfr_support: Option<XfrSupport>,
    pub max_memory_channels: Option<MaxMemoryChannels>,
    pub max_memory_frequency: Option<MaxMemoryFrequency>,
    pub compatable_chipsets: Option<CompatableChipsets>,

    pub performance_core_base_frequency: Option<PerformanceCoreBaseFrequency>,
    pub efficient_core_base_frequency: Option<EfficientCoreBaseFrequency>,
    pub performance_core_boost_frequency: Option<PerformanceCoreBoostFrequency>,
    pub efficient_core_boost_frequency: Option<EfficientCoreBoostFrequency>,
    pub performance_core_count: Option<PerformanceCoreCount>,
    pub efficient_core_count: Option<EfficientCoreCount>,
    pub performance_thread_count: Option<PerformanceThreadCount>,
    pub efficient_thread_count: Option<EfficientThreadCount>,
    pub ctdp_support: Option<CtdpSupport>,
    pub efficient_core_architecture: Option<EfficientCoreArchitecture>,
    // Shared
    pub manufacturer: Option<Manufacturer>,
    pub market: Option<Market>,
    pub architecture: Option<Architecture>,
    pub lithography: Option<Lithography>,
    pub release_date: Option<ReleaseDate>,
}
impl SpecDbType for Cpu {
    fn from_yaml(data: &Yaml) -> Self {
        let core_count = data["Core Count"].as_i64().expect("Core Count is required for type Cpu");
        let thread_count = data["Thread Count"].as_i64().expect("Thread count is required for type Cpu");
        let base_frequency = data["Base Frequency"].as_str().expect("Base Frequency is required for type Cpu").to_string();
        let tdp = data["TDP"].as_str().expect("TDP is required for type Cpu").to_string();
        Cpu {
            core_count: CoreCount (u16::try_from(core_count).expect("Core Count too large")),
            thread_count: ThreadCount (u16::try_from(thread_count).expect("Core Count too large")),
            base_frequency: BaseFrequency (base_frequency),
            tdp: Tdp (tdp),
            manufacturer: Manufacturer::from_yaml(data),
            architecture: Architecture::from_yaml(data),
            lithography: Lithography::from_yaml(data),
            release_date: ReleaseDate::from_yaml(data),
            market: Market::from_yaml(data),
            boost_frequency: BoostFrequency::from_yaml(data),
            xfr_frequency: XfrFrequency::from_yaml(data),
            socket: Socket::from_yaml(data),
            stepping: Stepping::from_yaml(data),
            l1_cache_data: L1CacheData::from_yaml(data),
            l1_cache_instruction: L1CacheInstruction::from_yaml(data),
            l2_cache_total: L2CacheTotal::from_yaml(data),
            l3_cache_total: L3CacheTotal::from_yaml(data),
            memory_type: MemoryType::from_yaml(data),
            pcie_5_0_lanes: Pcie50Lanes::from_yaml(data),
            pcie_4_0_lanes: Pcie40Lanes::from_yaml(data),
            pcie_3_0_lanes: Pcie30Lanes::from_yaml(data),
            pcie_2_0_lanes: Pcie20Lanes::from_yaml(data),
            pcie_1_0_lanes: Pcie10Lanes::from_yaml(data),
            avx_sse_mmx: AvxSseMmx::from_yaml(data),
            fma4: Fma4::from_yaml(data),
            fma3: Fma3::from_yaml(data),
            bmi: Bmi::from_yaml(data),
            aes: Aes::from_yaml(data),
            sha: Sha::from_yaml(data),
            other_extensions: OtherExtensions::from_yaml(data),
            unlocked: Unlocked::from_yaml(data),
            xfr_support: XfrSupport::from_yaml(data),
            max_memory_channels: MaxMemoryChannels::from_yaml(data),
            max_memory_frequency: MaxMemoryFrequency::from_yaml(data),
            compatable_chipsets: CompatableChipsets::from_yaml(data),
            performance_core_base_frequency: PerformanceCoreBaseFrequency::from_yaml(data),
            efficient_core_base_frequency: EfficientCoreBaseFrequency::from_yaml(data),
            performance_core_boost_frequency: PerformanceCoreBoostFrequency::from_yaml(data),
            efficient_core_boost_frequency: EfficientCoreBoostFrequency::from_yaml(data),
            performance_core_count: PerformanceCoreCount::from_yaml(data),
            efficient_core_count: EfficientCoreCount::from_yaml(data),
            performance_thread_count: PerformanceThreadCount::from_yaml(data),
            efficient_thread_count: EfficientThreadCount::from_yaml(data),
            ctdp_support: CtdpSupport::from_yaml(data),
            efficient_core_architecture: EfficientCoreArchitecture::from_yaml(data),
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
            core_count: CoreCount (u16::try_from(core_count).expect("Core Count too large")),
            thread_count: ThreadCount (u16::try_from(thread_count).expect("Core Count too large")),
            base_frequency: BaseFrequency (base_frequency),
            tdp: Tdp (tdp),
            manufacturer: Manufacturer::from_hashmap(&data),
            market: Market::from_hashmap(&data),
            architecture: Architecture::from_hashmap(&data),
            lithography: Lithography::from_hashmap(&data),
            release_date: ReleaseDate::from_hashmap(&data),
            boost_frequency: BoostFrequency::from_hashmap(&data),
            xfr_frequency: XfrFrequency::from_hashmap(&data),
            socket: Socket::from_hashmap(&data),
            stepping: Stepping::from_hashmap(&data),
            l1_cache_data: L1CacheData::from_hashmap(&data),
            l1_cache_instruction: L1CacheInstruction::from_hashmap(&data),
            l2_cache_total: L2CacheTotal::from_hashmap(&data),
            l3_cache_total: L3CacheTotal::from_hashmap(&data),
            memory_type: MemoryType::from_hashmap(&data),
            pcie_5_0_lanes: Pcie50Lanes::from_hashmap(&data),
            pcie_4_0_lanes: Pcie40Lanes::from_hashmap(&data),
            pcie_3_0_lanes: Pcie30Lanes::from_hashmap(&data),
            pcie_2_0_lanes: Pcie20Lanes::from_hashmap(&data),
            pcie_1_0_lanes: Pcie10Lanes::from_hashmap(&data),
            avx_sse_mmx: AvxSseMmx::from_hashmap(&data),
            fma4: Fma4::from_hashmap(&data),
            fma3: Fma3::from_hashmap(&data),
            bmi: Bmi::from_hashmap(&data),
            aes: Aes::from_hashmap(&data),
            sha: Sha::from_hashmap(&data),
            other_extensions: OtherExtensions::from_hashmap(&data),
            unlocked: Unlocked::from_hashmap(&data),
            xfr_support: XfrSupport::from_hashmap(&data),
            max_memory_channels: MaxMemoryChannels::from_hashmap(&data),
            max_memory_frequency: MaxMemoryFrequency::from_hashmap(&data),
            compatable_chipsets: CompatableChipsets::from_hashmap(&data),
            performance_core_base_frequency: PerformanceCoreBaseFrequency::from_hashmap(&data),
            efficient_core_base_frequency: EfficientCoreBaseFrequency::from_hashmap(&data),
            performance_core_boost_frequency: PerformanceCoreBoostFrequency::from_hashmap(&data),
            efficient_core_boost_frequency: EfficientCoreBoostFrequency::from_hashmap(&data),
            performance_core_count: PerformanceCoreCount::from_hashmap(&data),
            efficient_core_count: EfficientCoreCount::from_hashmap(&data),
            performance_thread_count: PerformanceThreadCount::from_hashmap(&data),
            efficient_thread_count: EfficientThreadCount::from_hashmap(&data),
            ctdp_support: CtdpSupport::from_hashmap(&data),
            efficient_core_architecture: EfficientCoreArchitecture::from_hashmap(&data),
        }
    }
}