// noinspection JSUnusedGlobalSymbols

export const module : {    
    readonly memory: WebAssembly.Memory;
    
    /**
     * - WASM type: `int`
     * - Rust name: `simulator::_::PANIC_MSG`
     * - Rust type: `[u8 ; 1024]`
     */
    readonly panicBuffer: number;
    
    /**
     * - WASM type: `int`
     * - Rust name: `simulator::wasm::globals::VOLATILE_PARAMS`
     * - Rust type: {@link SimulatorParams `SimulatorParams`}
     */
    readonly paramsPtr: number;
    
    /**
     * - WASM type: `int`
     * - Rust name: `simulator::wasm::globals::SIMULATOR`
     * - Rust type: {@link Mutex_Option_Simulator `Mutex<Option<Simulator>>`}
     */
    readonly simulatorPtr: number;
    
    /**
     * - WASM type: `int`
     * - Rust name: `simulator::_::PANIC_MSG_BUFFER_LEN`
     * - Rust type: `usize`
     */
    readonly panicBufferLen: number;
    
    /**
     * - Rust return type: {@link Simulator `Simulator`}
     * - Rust name: `simulator::wasm::globals::alloc_simulator`
     * @param a
     *      - Converted from RVO return value
     *      - WASM type: `{ a:int, b:int, c:int, d:int, e:int, f:int }`
     *      - Rust type: {@link Simulator `&mut Simulator`}
     */
    allocSimulator(a: number): void;
    
    /**
     * - Rust return type: `()`
     * - Rust name: `simulator::_::clear_panic_message`
     */
    clearPanicBuffer(): void;
    
    /**
     * - Rust return type: `()`
     * - Rust name: `simulator::wasm::globals::update_simulator_params`
     */
    updateParams(): void;
}

export namespace types {    
    /* Dummy helper type for syntactically indicating the types referenced by a type */
    type $<_ extends any[]> = never;
    
    /**
     * - Name: `SimulatorParams`
     * - Size: `11`
     * 
     * Fields:
     * | Name                | @   | Type                                                  |
     * | ------------------- | --- | ----------------------------------------------------- |
     * | `cache_params`      | `0` | {@link CacheParams `CacheParams`}                     |
     * | `pipeline_params`   | `5` | {@link PipelineParams `PipelineParams`}               |
     * | `branch_prediction` | `8` | {@link BranchPredictorParams `BranchPredictorParams`} |
     */
    type SimulatorParams = $<[CacheParams, PipelineParams, BranchPredictorParams]>;
    
    /**
     * - Name: `Mutex<Option<Simulator>>`
     * - Size: `1056`
     * 
     * Fields:
     * | Name     | @      | Type                                                                |
     * | -------- | ------ | ------------------------------------------------------------------- |
     * | `data`   | `0`    | {@link UnsafeCell_Option_Simulator `UnsafeCell<Option<Simulator>>`} |
     * | `locked` | `1048` | {@link UnsafeCell_bool `UnsafeCell<bool>`}                          |
     */
    type Mutex_Option_Simulator = $<[UnsafeCell_Option_Simulator, UnsafeCell_bool]>;
    
    /**
     * - Name: `Simulator`
     * - Size: `1048`
     * 
     * Fields:
     * | Name               | @    | Type                                                                             |
     * | ------------------ | ---- | -------------------------------------------------------------------------------- |
     * | `pipeline`         | `0`  | {@link WASMAllocation_dyn_Pipeline `WASMAllocation<dyn Pipeline>`}               |
     * | `cache`            | `8`  | {@link WASMAllocation_dyn_MemoryAccess `WASMAllocation<dyn MemoryAccess>`}       |
     * | `branch_predictor` | `16` | {@link WASMAllocation_dyn_BranchPredictor `WASMAllocation<dyn BranchPredictor>`} |
     * | `registers`        | `24` | `[u64 ; 128]`                                                                    |
     */
    type Simulator = $<[WASMAllocation_dyn_Pipeline, WASMAllocation_dyn_MemoryAccess, WASMAllocation_dyn_BranchPredictor]>;
    
    /**
     * - Name: `CacheParams`
     * - Size: `5`
     * 
     * Fields:
     * | Name             | @   | Type                                    |
     * | ---------------- | --- | --------------------------------------- |
     * | `associativity`  | `0` | `u8`                                    |
     * | `block_size_log` | `1` | `u8`                                    |
     * | `num_sets_log`   | `2` | `u8`                                    |
     * | `policy`         | `3` | {@link CachePolicy `CachePolicy`}       |
     * | `write_mode`     | `4` | {@link CacheWriteMode `CacheWriteMode`} |
     */
    type CacheParams = $<[CachePolicy, CacheWriteMode]>;
    
    /**
     * - Name: `PipelineParams`
     * - Size: `3`
     * 
     * Fields:
     * | Name            | @   | Type                                      |
     * | --------------- | --- | ----------------------------------------- |
     * | `pipeline_mode` | `0` | {@link PipelineMode `PipelineMode`}       |
     * | `cycle_times`   | `1` | {@link CycleTimeParams `CycleTimeParams`} |
     */
    type PipelineParams = $<[PipelineMode, CycleTimeParams]>;
    
    /**
     * - Name: `BranchPredictorParams`
     * - Size: `3`
     * 
     * Fields:
     * | Name                | @   | Type                                                            |
     * | ------------------- | --- | --------------------------------------------------------------- |
     * | `bht_size_log`      | `0` | `u8`                                                            |
     * | `dynamic_predictor` | `1` | {@link DynamicBranchPredictor `DynamicBranchPredictor`}         |
     * | `static_mode`       | `2` | {@link StaticBranchPredictionMode `StaticBranchPredictionMode`} |
     */
    type BranchPredictorParams = $<[DynamicBranchPredictor, StaticBranchPredictionMode]>;
    
    /**
     * - Name: `UnsafeCell<Option<Simulator>>`
     * - Size: `1048`
     * 
     * Fields:
     * | Name    | @   | Type                                         |
     * | ------- | --- | -------------------------------------------- |
     * | `value` | `0` | {@link Option_Simulator `Option<Simulator>`} |
     */
    type UnsafeCell_Option_Simulator = $<[Option_Simulator]>;
    
    /**
     * - Name: `UnsafeCell<bool>`
     * - Size: `1`
     * 
     * Fields:
     * | Name    | @   | Type   |
     * | ------- | --- | ------ |
     * | `value` | `0` | `bool` |
     */
    type UnsafeCell_bool = $<[]>;
    
    /**
     * - Name: `WASMAllocation<dyn Pipeline>`
     * - Size: `8`
     * 
     * Fields:
     * | Name  | @   | Type                                             |
     * | ----- | --- | ------------------------------------------------ |
     * | `ptr` | `0` | {@link ptr_mut_dyn_Pipeline `*mut dyn Pipeline`} |
     */
    type WASMAllocation_dyn_Pipeline = $<[ptr_mut_dyn_Pipeline]>;
    
    /**
     * - Name: `WASMAllocation<dyn MemoryAccess>`
     * - Size: `8`
     * 
     * Fields:
     * | Name  | @   | Type                                                     |
     * | ----- | --- | -------------------------------------------------------- |
     * | `ptr` | `0` | {@link ptr_mut_dyn_MemoryAccess `*mut dyn MemoryAccess`} |
     */
    type WASMAllocation_dyn_MemoryAccess = $<[ptr_mut_dyn_MemoryAccess]>;
    
    /**
     * - Name: `WASMAllocation<dyn BranchPredictor>`
     * - Size: `8`
     * 
     * Fields:
     * | Name  | @   | Type                                                           |
     * | ----- | --- | -------------------------------------------------------------- |
     * | `ptr` | `0` | {@link ptr_mut_dyn_BranchPredictor `*mut dyn BranchPredictor`} |
     */
    type WASMAllocation_dyn_BranchPredictor = $<[ptr_mut_dyn_BranchPredictor]>;
    
    /**
     * - Name: `CachePolicy`
     * - Size: `1`
     * - Discriminant layout: `u8` @ `0`
     * 
     * Variants:
     * 
     * - `LRU` (discriminant value = `<default>`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `NMRU` (discriminant value = `1`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     */
    type CachePolicy = $<[]>;
    
    /**
     * - Name: `CacheWriteMode`
     * - Size: `1`
     * - Discriminant layout: `u8` @ `0`
     * 
     * Variants:
     * 
     * - `WriteBack` (discriminant value = `<default>`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `WriteThrough` (discriminant value = `1`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     */
    type CacheWriteMode = $<[]>;
    
    /**
     * - Name: `PipelineMode`
     * - Size: `1`
     * - Discriminant layout: `u8` @ `0`
     * 
     * Variants:
     * 
     * - `None` (discriminant value = `<default>`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `ThreeStage` (discriminant value = `1`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `FiveStage` (discriminant value = `2`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     */
    type PipelineMode = $<[]>;
    
    /**
     * - Name: `CycleTimeParams`
     * - Size: `2`
     * 
     * Fields:
     * | Name           | @   | Type |
     * | -------------- | --- | ---- |
     * | `cache_access` | `0` | `u8` |
     * | `dram_penalty` | `1` | `u8` |
     */
    type CycleTimeParams = $<[]>;
    
    /**
     * - Name: `DynamicBranchPredictor`
     * - Size: `1`
     * - Discriminant layout: `u8` @ `0`
     * 
     * Variants:
     * 
     * - `None` (discriminant value = `<default>`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `OneBitSaturating` (discriminant value = `1`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `TwoBitSaturating` (discriminant value = `2`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     */
    type DynamicBranchPredictor = $<[]>;
    
    /**
     * - Name: `StaticBranchPredictionMode`
     * - Size: `1`
     * - Discriminant layout: `u8` @ `0`
     * 
     * Variants:
     * 
     * - `Always` (discriminant value = `<default>`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `Never` (discriminant value = `1`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `Directional` (discriminant value = `2`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     */
    type StaticBranchPredictionMode = $<[]>;
    
    /**
     * - Name: `Option<Simulator>`
     * - Size: `1048`
     * - Discriminant layout: `u32` @ `4`
     * 
     * Variants:
     * 
     * - `None` (discriminant value = `0`):
     * | Name | @ | Type |
     * | ---- | - | ---- |
     * 
     * - `Some` (discriminant value = `<default>`):
     * | Name  | @   | Type                          |
     * | ----- | --- | ----------------------------- |
     * | `__0` | `0` | {@link Simulator `Simulator`} |
     */
    type Option_Simulator = $<[Simulator]>;
    
    /**
     * - Name: `*mut dyn Pipeline`
     * - Size: `8`
     * 
     * Fields:
     * | Name      | @   | Type                                             |
     * | --------- | --- | ------------------------------------------------ |
     * | `pointer` | `0` | {@link ptr_mut_dyn_Pipeline `*mut dyn Pipeline`} |
     * | `vtable`  | `4` | `&[usize ; 5]`                                   |
     * 
     * `Pipeline` implementations: {@link DummyPipeline `DummyPipeline`}
     */
    type ptr_mut_dyn_Pipeline = $<[ptr_mut_dyn_Pipeline, DummyPipeline]>;
    
    /**
     * - Name: `*mut dyn MemoryAccess`
     * - Size: `8`
     * 
     * Fields:
     * | Name      | @   | Type                                                     |
     * | --------- | --- | -------------------------------------------------------- |
     * | `pointer` | `0` | {@link ptr_mut_dyn_MemoryAccess `*mut dyn MemoryAccess`} |
     * | `vtable`  | `4` | `&[usize ; 12]`                                          |
     * 
     * `MemoryAccess` implementations: {@link DirectMemoryAccess `DirectMemoryAccess`}
     */
    type ptr_mut_dyn_MemoryAccess = $<[ptr_mut_dyn_MemoryAccess, DirectMemoryAccess]>;
    
    /**
     * - Name: `*mut dyn BranchPredictor`
     * - Size: `8`
     * 
     * Fields:
     * | Name      | @   | Type                                                           |
     * | --------- | --- | -------------------------------------------------------------- |
     * | `pointer` | `0` | {@link ptr_mut_dyn_BranchPredictor `*mut dyn BranchPredictor`} |
     * | `vtable`  | `4` | `&[usize ; 6]`                                                 |
     * 
     * `BranchPredictor` implementations: {@link Predictor0 `Predictor0`}, {@link Predictor1 `Predictor1`}, {@link Predictor2 `Predictor2`}
     */
    type ptr_mut_dyn_BranchPredictor = $<[ptr_mut_dyn_BranchPredictor, Predictor0, Predictor1, Predictor2]>;
    
    /**
     * - Name: `DummyPipeline`
     * - Size: `0`
     */
    type DummyPipeline = $<[]>;
    
    /**
     * - Name: `DirectMemoryAccess`
     * - Size: `131072`
     * 
     * Fields:
     * | Name     | @   | Type                                    |
     * | -------- | --- | --------------------------------------- |
     * | `memory` | `0` | {@link GrowableMemory `GrowableMemory`} |
     */
    type DirectMemoryAccess = $<[GrowableMemory]>;
    
    /**
     * - Name: `Predictor0`
     * - Size: `20`
     * 
     * Fields:
     * | Name   | @   | Type                                                   |
     * | ------ | --- | ------------------------------------------------------ |
     * | `base` | `0` | {@link BranchPredictorBase_4 `BranchPredictorBase<4>`} |
     */
    type Predictor0 = $<[BranchPredictorBase_4]>;
    
    /**
     * - Name: `Predictor1`
     * - Size: `20`
     * 
     * Fields:
     * | Name   | @   | Type                                                   |
     * | ------ | --- | ------------------------------------------------------ |
     * | `base` | `0` | {@link BranchPredictorBase_4 `BranchPredictorBase<4>`} |
     */
    type Predictor1 = $<[BranchPredictorBase_4]>;
    
    /**
     * - Name: `Predictor2`
     * - Size: `20`
     * 
     * Fields:
     * | Name   | @   | Type                                                   |
     * | ------ | --- | ------------------------------------------------------ |
     * | `base` | `0` | {@link BranchPredictorBase_4 `BranchPredictorBase<4>`} |
     */
    type Predictor2 = $<[BranchPredictorBase_4]>;
    
    /**
     * - Name: `GrowableMemory`
     * - Size: `131072`
     * 
     * Fields:
     * | Name         | @   | Type            |
     * | ------------ | --- | --------------- |
     * | `page_table` | `0` | `[u16 ; 65536]` |
     */
    type GrowableMemory = $<[]>;
    
    /**
     * - Name: `BranchPredictorBase<4>`
     * - Size: `20`
     * 
     * Fields:
     * | Name                   | @    | Type                                                                                                             |
     * | ---------------------- | ---- | ---------------------------------------------------------------------------------------------------------------- |
     * | `branch_target_buffer` | `0`  | {@link WASMAllocation_FixedSizeLRUCache_BTBCacheEntry_4 `WASMAllocation<[FixedSizeLRUCache<BTBCacheEntry, 4>]>`} |
     * | `branch_history_table` | `8`  | {@link WASMAllocation_u8 `WASMAllocation<[u8]>`}                                                                 |
     * | `static_mode`          | `16` | {@link StaticBranchPredictionMode `StaticBranchPredictionMode`}                                                  |
     */
    type BranchPredictorBase_4 = $<[WASMAllocation_FixedSizeLRUCache_BTBCacheEntry_4, WASMAllocation_u8, StaticBranchPredictionMode]>;
    
    /**
     * - Name: `WASMAllocation<[FixedSizeLRUCache<BTBCacheEntry, 4>]>`
     * - Size: `8`
     * 
     * Fields:
     * | Name  | @   | Type                                                                                           |
     * | ----- | --- | ---------------------------------------------------------------------------------------------- |
     * | `ptr` | `0` | {@link ptr_mut_FixedSizeLRUCache_BTBCacheEntry_4 `*mut [FixedSizeLRUCache<BTBCacheEntry, 4>]`} |
     */
    type WASMAllocation_FixedSizeLRUCache_BTBCacheEntry_4 = $<[ptr_mut_FixedSizeLRUCache_BTBCacheEntry_4]>;
    
    /**
     * - Name: `WASMAllocation<[u8]>`
     * - Size: `8`
     * 
     * Fields:
     * | Name  | @   | Type                           |
     * | ----- | --- | ------------------------------ |
     * | `ptr` | `0` | {@link ptr_mut_u8 `*mut [u8]`} |
     */
    type WASMAllocation_u8 = $<[ptr_mut_u8]>;
    
    /**
     * - Name: `*mut [FixedSizeLRUCache<BTBCacheEntry, 4>]`
     * - Size: `8`
     * 
     * Fields:
     * | Name       | @   | Type                                                                                 |
     * | ---------- | --- | ------------------------------------------------------------------------------------ |
     * | `data_ptr` | `0` | {@link FixedSizeLRUCache_BTBCacheEntry_4 `*mut FixedSizeLRUCache<BTBCacheEntry, 4>`} |
     * | `length`   | `4` | `usize`                                                                              |
     */
    type ptr_mut_FixedSizeLRUCache_BTBCacheEntry_4 = $<[FixedSizeLRUCache_BTBCacheEntry_4]>;
    
    /**
     * - Name: `*mut [u8]`
     * - Size: `8`
     * 
     * Fields:
     * | Name       | @   | Type      |
     * | ---------- | --- | --------- |
     * | `data_ptr` | `0` | `*mut u8` |
     * | `length`   | `4` | `usize`   |
     */
    type ptr_mut_u8 = $<[]>;
    
    /**
     * - Name: `FixedSizeLRUCache<BTBCacheEntry, 4>`
     * - Size: `72`
     * 
     * Fields:
     * | Name    | @    | Type                                                                |
     * | ------- | ---- | ------------------------------------------------------------------- |
     * | `data`  | `0`  | {@link Array_BTBCacheEntry_4 `Array<BTBCacheEntry, 4>`}             |
     * | `state` | `64` | {@link UnsafeCell_MatrixLRUState_4 `UnsafeCell<MatrixLRUState<4>>`} |
     */
    type FixedSizeLRUCache_BTBCacheEntry_4 = $<[Array_BTBCacheEntry_4, UnsafeCell_MatrixLRUState_4]>;
    
    /**
     * - Name: `Array<BTBCacheEntry, 4>`
     * - Size: `64`
     * 
     * Fields:
     * | Name  | @   | Type                                        |
     * | ----- | --- | ------------------------------------------- |
     * | `__0` | `0` | {@link BTBCacheEntry `[BTBCacheEntry ; 4]`} |
     */
    type Array_BTBCacheEntry_4 = $<[BTBCacheEntry]>;
    
    /**
     * - Name: `UnsafeCell<MatrixLRUState<4>>`
     * - Size: `8`
     * 
     * Fields:
     * | Name    | @   | Type                                         |
     * | ------- | --- | -------------------------------------------- |
     * | `value` | `0` | {@link MatrixLRUState_4 `MatrixLRUState<4>`} |
     */
    type UnsafeCell_MatrixLRUState_4 = $<[MatrixLRUState_4]>;
    
    /**
     * - Name: `BTBCacheEntry`
     * - Size: `16`
     * 
     * Fields:
     * | Name     | @   | Type  |
     * | -------- | --- | ----- |
     * | `tag`    | `0` | `u64` |
     * | `target` | `8` | `u64` |
     */
    type BTBCacheEntry = $<[]>;
    
    /**
     * - Name: `MatrixLRUState<4>`
     * - Size: `8`
     * 
     * Fields:
     * | Name     | @   | Type                                   |
     * | -------- | --- | -------------------------------------- |
     * | `matrix` | `0` | `u64`                                  |
     * | `_n`     | `8` | {@link PhantomData_4 `PhantomData<4>`} |
     */
    type MatrixLRUState_4 = $<[PhantomData_4]>;
    
    /**
     * - Name: `PhantomData<4>`
     * - Size: `0`
     */
    type PhantomData_4 = $<[]>;
}

export default module;