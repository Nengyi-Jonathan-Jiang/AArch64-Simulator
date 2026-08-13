use crate::alloc_interface::{allocation_to_trait_object, IAlloc, IAllocation};
use crate::components::mem_access::DirectMemoryAccess;
use crate::components::pipeline::DummyPipeline;
use crate::components::{branch_predictor, BranchPredictor, MemoryAccess, Pipeline, Simulator};
use crate::params::{BranchPredictorParams, CacheParams, CachePolicy, DynamicBranchPredictor, PipelineParams, SimulatorParams};
use crate::wasm::wasm_mutex::Mutex;
use crate::{Alloc, Allocation};
use core::mem;

// Compiler puts this in a "readonly" section, but it doesn't actually matter because the underlying
// WASM memory, being a normal ArrayBuffer, is writable by JS
#[unsafe(export_name = "paramsPtr")]
static VOLATILE_PARAMS: SimulatorParams = unsafe { mem::zeroed() };

static SAVED_PARAMS: Mutex<SimulatorParams> = Mutex::new(VOLATILE_PARAMS);

#[unsafe(export_name = "simulatorPtr")]
static SIMULATOR: Mutex<Option<Simulator>> = Mutex::new(None);

#[unsafe(export_name = "updateParams")]
unsafe fn update_simulator_params() {
    unsafe {
        // Update saved params from volatile memory
        *SAVED_PARAMS.lock() = (&raw const VOLATILE_PARAMS).read_volatile();
        // Set simulator back to None so next access can reinitialize it with the new params
        dealloc_simulator();
    }
}

unsafe fn dealloc_simulator() {
    // Reset dynamic memory held by simulator
    Alloc::reset();
    // Erase simulator data
    *SIMULATOR.lock() = None;
}

fn alloc_pipeline(params: PipelineParams) -> Allocation<dyn Pipeline> {
    let PipelineParams {
        pipeline_mode: _,
        cycle_times: _,
    } = params;

    // TODO: un-dummy this
    DummyPipeline::new()

    // match pipeline_mode {
    //     PipelineMode::None => {}
    //     PipelineMode::ThreeStage => {}
    //     PipelineMode::FiveStage => {}
    // }
}

fn alloc_cache(params: CacheParams) -> Allocation<dyn MemoryAccess> {
    let CacheParams {
        associativity,
        block_size_log,
        capacity_log,
        policy,
        write_mode,
    } = params;

    // TODO: un-dummy this
    match policy {
        CachePolicy::None => {
            allocation_to_trait_object!(
                Alloc::alloc::<DirectMemoryAccess>().init_zeroed()
                =>
                MemoryAccess
            )
        }
        CachePolicy::LRU => {
            todo!()
        }
        CachePolicy::NMRU => {
            todo!()
        }
    }
}

fn alloc_branch_predictor(params: BranchPredictorParams) -> Allocation<dyn BranchPredictor> {
    use crate::components::branch_predictor::ConstructBranchPredictor;

    let BranchPredictorParams {
        bht_size_log,
        dynamic_predictor,
        static_mode,
    } = params;

    match dynamic_predictor {
        DynamicBranchPredictor::None => {
            branch_predictor::Predictor0::new(16, bht_size_log, static_mode)
        }
        DynamicBranchPredictor::OneBitSaturating => {
            branch_predictor::Predictor1::new(16, bht_size_log, static_mode)
        }
        DynamicBranchPredictor::TwoBitSaturating => {
            branch_predictor::Predictor2::new(16, bht_size_log, static_mode)
        }
    }
}

#[unsafe(export_name = "allocSimulator")]
fn alloc_simulator() -> Simulator {
    let SimulatorParams {
        cache_params,
        pipeline_params,
        branch_prediction,
    } = *SAVED_PARAMS.lock();

    Simulator::new(
        alloc_pipeline(pipeline_params),
        alloc_cache(cache_params),
        alloc_branch_predictor(branch_prediction),
    )
}

fn with_simulator<F: FnOnce(&mut Simulator) -> ()>(f: F) {
    let mut simulator_guard = SIMULATOR.lock();

    let simulator = simulator_guard.get_or_insert_with(|| alloc_simulator());
    f(simulator);

    drop(simulator_guard); // Explicitly release the lock
}
