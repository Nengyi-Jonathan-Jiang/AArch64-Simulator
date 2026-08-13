use crate::components::sizes::Addr;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SimulatorParams {
    pub cache_params: CacheParams,
    pub pipeline_params: PipelineParams,
    pub branch_prediction: BranchPredictorParams,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CacheParams {
    pub associativity: u8,
    pub block_size_log: u8,
    pub capacity_log: u8,

    pub policy: CachePolicy,
    pub write_mode: CacheWriteMode,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PipelineParams {
    pub pipeline_mode: PipelineMode,

    pub cycle_times: CycleTimeParams,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CycleTimeParams {
    pub cache_access: u8,
    pub dram_penalty: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct BranchPredictorParams {
    pub bht_size_log: u8,
    pub dynamic_predictor: DynamicBranchPredictor,
    pub static_mode: StaticBranchPredictionMode,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CachePolicy {
    None = 0,
    LRU = 1,
    NMRU = 2,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CacheWriteMode {
    // Data is written to main memory when the cache line is evicted
    WriteBack = 0,
    // Data is written to main memory immediately upon write
    WriteThrough = 1,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PipelineMode {
    None = 0,
    ThreeStage = 1,
    FiveStage = 2,
}

#[repr(u8)]
#[derive(Copy, Clone, Default)]
pub enum StaticBranchPredictionMode {
    /// Conditional branches are predicted to always be taken
    #[default]
    Always = 0,
    /// Conditional branches are predicted to never be taken
    Never = 1,
    /// Conditional branches are predicted to be taken if backwards (predicted target < addr)
    Directional = 2,
}

impl StaticBranchPredictionMode {
    pub fn static_predict(&self, addr: Addr, target: Addr) -> bool {
        match self {
            StaticBranchPredictionMode::Always => true,
            StaticBranchPredictionMode::Never => false,
            StaticBranchPredictionMode::Directional => addr > target,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DynamicBranchPredictor {
    None = 0,
    OneBitSaturating = 1,
    TwoBitSaturating = 2,
}
