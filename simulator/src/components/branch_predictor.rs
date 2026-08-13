use crate::alloc_interface::{IAlloc, IAllocation};
use crate::components::BranchPredictionResult::{NotTaken, Taken};
use crate::components::lru_cache::{FixedSizeLRUCache, LRUCache};
use crate::components::sizes::{Addr, INSTRUCTION_SIZE_BYTES, Instruction};
use crate::components::{BranchPredictionResult, BranchPredictor};
use crate::params::StaticBranchPredictionMode;
use crate::{Alloc, Allocation};
use hybrid_array::ArraySize;
use typenum::{U0, U1, U2, U4, Unsigned};

#[repr(transparent)]
pub struct Predictor0 {
    base: BranchPredictorBase<<Self as UseBranchPredictorBase>::BTBAssociativity>,
}

unsafe impl UseBranchPredictorBase for Predictor0 {
    type BTBAssociativity = U4; // 4 is good enough
    type BHTEntrySizeBits = U0;
    const INITIAL_BYTE: u8 = 0; // Irrelevant, 0 is fine

    fn dynamic_predict(_: u8, static_prediction: bool) -> bool {
        static_prediction
    }

    fn dynamic_update(_: u8, _: bool) -> u8 {
        0
    }

    fn base(&self) -> &BranchPredictorBase<Self::BTBAssociativity> {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BranchPredictorBase<Self::BTBAssociativity> {
        &mut self.base
    }
}

#[repr(transparent)]
pub struct Predictor1 {
    base: BranchPredictorBase<<Self as UseBranchPredictorBase>::BTBAssociativity>,
}

unsafe impl UseBranchPredictorBase for Predictor1 {
    type BTBAssociativity = U4; // 4 is good enough
    type BHTEntrySizeBits = U1;
    const INITIAL_BYTE: u8 = 1; // 1 = predict taken

    fn dynamic_predict(state: u8, _: bool) -> bool {
        state != 0
    }

    fn dynamic_update(_: u8, did_jump: bool) -> u8 {
        did_jump as u8
    }

    fn base(&self) -> &BranchPredictorBase<Self::BTBAssociativity> {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BranchPredictorBase<Self::BTBAssociativity> {
        &mut self.base
    }
}

#[repr(transparent)]
pub struct Predictor2 {
    base: BranchPredictorBase<<Self as UseBranchPredictorBase>::BTBAssociativity>,
}

unsafe impl UseBranchPredictorBase for Predictor2 {
    type BTBAssociativity = U4; // 4 is good enough
    type BHTEntrySizeBits = U2;
    const INITIAL_BYTE: u8 = 2; // 2 = slightly predict taken

    fn dynamic_predict(state: u8, _: bool) -> bool {
        (state & 2) != 0
    }

    fn dynamic_update(mut state: u8, did_jump: bool) -> u8 {
        // new_state = state + {1 if did_jump else -1} = state + 2 * did_jump - 1
        state = state + 2 * (did_jump as u8); // so this is new_state + 1

        if state == 0 {
            0 // new_state + 1 == 0 so new_state == -1, clamp to 0
        } else if state == 5 {
            3 // new_state + 1 == 5 so new_state == 4, clamp to 3
        } else {
            state - 1 // return new_state
        }
    }

    fn base(&self) -> &BranchPredictorBase<Self::BTBAssociativity> {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BranchPredictorBase<Self::BTBAssociativity> {
        &mut self.base
    }
}

/// NOT ZeroInit
struct BTBCacheEntry {
    tag: Addr,
    target: Addr,
}

impl Default for BTBCacheEntry {
    fn default() -> Self {
        Self {
            tag: 1,    // Never a valid tag
            target: 1, // Doesn't matter, our corresponding tag is never generated
        }
    }
}

struct BranchPredictorBase<
    // needs to use typenum instead of generic const because this needs to work with
    // UseBranchPredictorBase's associated type BTBAssociativity
    BTBAssociativity: ArraySize,
> {
    static_mode: StaticBranchPredictionMode,
    branch_target_buffer: Allocation<[FixedSizeLRUCache<BTBCacheEntry, BTBAssociativity>]>,
    branch_history_table: Allocation<[u8]>,
}

/// Provides a new() function that can be used to construct a BranchPredictor
pub trait ConstructBranchPredictor {
    /// Given a function that allocates memory and parameters, allocate and initialize an instance
    /// of self
    ///
    /// Unsafe:
    /// - `get_mem` must return a memory allocation with the given size and alignment in bytes that
    ///   satisfies the lifetime semantics of [`UnsafeMutRef`]
    fn new(
        btb_size_log: u8,
        bht_size_log: u8,
        static_mode: StaticBranchPredictionMode,
    ) -> Allocation<dyn BranchPredictor>;
}

/// Unsafe:
/// - The only non-zero-sized field in Self must be a BranchPredictorBase<EntryData>
unsafe trait UseBranchPredictorBase
where
    Self: Sized + BranchPredictor + 'static,
{
    /// Associativity of cache used in the branch target buffer
    type BTBAssociativity: ArraySize;
    /// Size of a BHT entry, in bits. This must divide 8 (so that there is an integer number of
    /// entries per byte)
    type BHTEntrySizeBits: ArraySize;

    /// Initial byte value used to bht memory
    const INITIAL_BYTE: u8;

    /// Given the bht state, predict whether the branch will be taken
    fn dynamic_predict(state: u8, static_prediction: bool) -> bool;
    /// Update the bht state with the given information (whether the branch was taken)
    fn dynamic_update(state: u8, did_jump: bool) -> u8;

    /// Get self as BranchPredictorBase
    fn base(&self) -> &BranchPredictorBase<Self::BTBAssociativity>;

    /// Get self as mut BranchPredictorBase
    fn base_mut(&mut self) -> &mut BranchPredictorBase<Self::BTBAssociativity>;
}

impl<T: UseBranchPredictorBase> BranchPredictor for T {
    fn predict(&self, addr: Addr) -> Option<BranchPredictionResult> {
        let base = self.base();

        let predicted_target_addr = {
            let btb = &base.branch_target_buffer;
            let BTBIndexer { index, tag } = index_btb(btb.len(), addr);

            btb[index].get(|x| x.tag == tag)?.target
        };

        let static_prediction = base.static_mode.static_predict(addr, predicted_target_addr);

        let bht_entry = if T::BHTEntrySizeBits::USIZE != 0 {
            let bht = &base.branch_history_table;
            let BHTIndexer {
                byte_index,
                mask,
                shift,
            } = index_bht::<T::BHTEntrySizeBits>(bht.len(), addr);

            (bht[byte_index] >> shift) & mask
        } else {
            0
        };

        Some(if T::dynamic_predict(bht_entry, static_prediction) {
            Taken(predicted_target_addr)
        } else {
            NotTaken
        })
    }

    fn update_target(&mut self, addr: Addr, target_addr: Addr) {
        let btb = self.base_mut().branch_target_buffer.as_mut();
        let BTBIndexer { index, tag } = index_btb(btb.len(), addr);

        btb[index]
            .get_or_insert(
                |x| x.tag == tag,
                |entry| {
                    entry.tag = tag;
                },
            )
            .target = target_addr;
    }

    fn update_branch(&mut self, addr: Addr, did_jump: bool) {
        if T::BHTEntrySizeBits::USIZE == 0 {
            return;
        }

        let bht = &mut self.base_mut().branch_history_table;
        let BHTIndexer {
            byte_index,
            mask,
            shift,
        } = index_bht::<T::BHTEntrySizeBits>(bht.len(), addr);

        let bht_entry = (bht[byte_index] >> shift) & mask;
        let new_entry = Self::dynamic_update(bht_entry, did_jump);
        bht[byte_index] &= !(mask << shift);
        bht[byte_index] |= new_entry << shift;
    }
}

struct BTBIndexer {
    /// The index in the btb corresponding to the address
    index: usize,
    /// The upper bits of the address not used for indexing
    tag: Addr,
}
struct BHTIndexer {
    /// The index of the byte to read from
    byte_index: usize,
    /// The bit position in the byte to index
    shift: usize,
    /// A mask for just the bits we are interested in
    mask: u8,
}

const fn index_btb(table_len: usize, addr: Addr) -> BTBIndexer {
    let addr = addr as usize / INSTRUCTION_SIZE_BYTES;
    let index = addr & (table_len - 1);
    let tag = (addr ^ (index * INSTRUCTION_SIZE_BYTES)) as Addr;

    BTBIndexer { index, tag }
}

const fn index_bht<EntrySizeBits: Unsigned>(bht_len: usize, addr: Addr) -> BHTIndexer {
    let index = (addr as usize / INSTRUCTION_SIZE_BYTES) & (bht_len - 1);

    let entries_per_byte = 8 / EntrySizeBits::USIZE;

    let byte_index = index / entries_per_byte;
    let mask = (1u8 << EntrySizeBits::USIZE) - 1;
    let shift = EntrySizeBits::USIZE * (index % entries_per_byte);

    BHTIndexer {
        byte_index,
        mask,
        shift,
    }
}

impl<T: UseBranchPredictorBase> ConstructBranchPredictor for T {
    fn new(
        btb_size_log: u8,
        bht_size_log: u8,
        static_mode: StaticBranchPredictionMode,
    ) -> Allocation<dyn BranchPredictor> {
        let btb_len = 1 << btb_size_log;
        let bht_len = (T::BHTEntrySizeBits::USIZE << bht_size_log).div_ceil(8);

        let branch_target_buffer =
            Alloc::alloc_slice::<FixedSizeLRUCache<BTBCacheEntry, T::BTBAssociativity>>(btb_len)
                .init_slice_with(|_| Default::default());
        let branch_history_table =
            Alloc::alloc_slice::<u8>(bht_len).map_slice(|_, _| T::INITIAL_BYTE);

        let res = Alloc::alloc::<BranchPredictorBase<T::BTBAssociativity>>().map(|_| {
            BranchPredictorBase {
                static_mode,
                branch_target_buffer,
                branch_history_table,
            }
        });

        unsafe {
            res.transmute::<T>()
                .unsized_map(|x: &mut T| x as &mut dyn BranchPredictor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// BTB cache index repeats per `(2 ** 4) * sizeof Instruction = 64` bytes, storing total
    /// `(2 ** 4) * (BTB Assoc) = 64` entries
    const BTB_SIZE_LOG: u8 = 4;
    /// BHT repeats per `(2 ** 8) * sizeof Instruction = 1024` bytes, storing total `2 ** 8 = 256`
    /// entries
    const BHT_SIZE_LOG: u8 = 8;

    macro check_predict {
        ($p:ident, $($e:tt -> $r: tt),+ $(,)?) => {
            $(
            check_predict!(@ $p, $e + 0, $r);
            )+
        },
        (@ $p:ident, $e:expr, ()) => {
            assert_eq!($p.predict($e), None)
        },
        (@ $p:ident, $e:expr, !) => {
            assert_eq!($p.predict($e), Some(NotTaken));
        },
        (@ $p:ident, $e:expr, $r: literal) => {
            assert_eq!($p.predict($e), Some(Taken($r)))
        }
    }

    #[test]
    fn test_predictor_0() {
        // Check always
        {
            let p = &mut *Predictor0::new(
                BTB_SIZE_LOG,
                BHT_SIZE_LOG,
                StaticBranchPredictionMode::Always,
            );

            // No entries are in the table
            check_predict!(p,
                0 -> (), 4 -> (), 8 -> (), 20 -> (), 264 -> (),
                (Addr::MAX & !(INSTRUCTION_SIZE_BYTES as Addr - 1)) -> (),
            );

            p.update_target(8, 12);

            check_predict!(p,
                0 -> (), 4 -> (),
                8 -> 12, // We just updated this one
                20 -> (),
                264 -> (), // Note that this has the same index of 8, but should still return nothing
                (Addr::MAX & !(INSTRUCTION_SIZE_BYTES as Addr - 1)) -> ()
            );

            p.update_target(8, 4);

            check_predict!(p,
                0 -> (), 4 -> (),
                8 -> 4, // We just updated this one
                20 -> (),
                264 -> (),
                (Addr::MAX & !(INSTRUCTION_SIZE_BYTES as Addr - 1)) -> ()
            );

            p.update_branch(8, false); // Should have no effect

            check_predict!(p, 8 -> 4);
        }

        // Check never
        {
            let p = &mut *Predictor0::new(
                BTB_SIZE_LOG,
                BHT_SIZE_LOG,
                StaticBranchPredictionMode::Never,
            );

            // Always predict not taken
            check_predict!(p,
                0 -> (), 4 -> (), 8 -> (), 20 -> (), 264 -> (),
                (Addr::MAX & !(INSTRUCTION_SIZE_BYTES as Addr - 1)) -> ()
            );

            p.update_target(8, 4);

            check_predict!(p, 8 -> !, 264 -> ());

            p.update_branch(8, true);

            check_predict!(p, 8 -> !, 264 -> ());
        }

        // Check directional
        {
            let p = &mut *Predictor0::new(
                BTB_SIZE_LOG,
                BHT_SIZE_LOG,
                StaticBranchPredictionMode::Directional,
            );

            check_predict!(p, 0 -> (), 8 -> (), 20 -> ());

            // 8 jump to 12 is forward, which should be predicted not taken
            p.update_target(8, 12);
            check_predict!(p, 8 -> !, 264 -> ());

            p.update_target(8, 4);

            // 8 now has a backward jump, which should be predicted taken
            check_predict!(p, 8 -> 4, 264 -> ());

            p.update_branch(8, false);

            // Shouldn't change the result
            check_predict!(p, 8 -> 4, 264 -> ());
        }
    }

    #[test]
    fn test_dynamic_predictor_1() {
        let p = &mut *Predictor1::new(
            BTB_SIZE_LOG,
            BHT_SIZE_LOG,
            StaticBranchPredictionMode::Directional,
        );

        check_predict!(p,
            0 -> (), 4 -> (), 8 -> (), 20 -> (), 264 -> (),
            (Addr::MAX & !(INSTRUCTION_SIZE_BYTES as Addr - 1)) -> (),
        );

        p.update_branch(8, true);

        // Should not have effect; btb wasn't updated
        check_predict!(p, 8 -> ());

        p.update_target(8, 24);

        check_predict!(p, 8 -> 24);
        // Nothing should change
        p.update_branch(8, true);
        check_predict!(p, 8 -> 24);
        p.update_branch(8, true);
        check_predict!(p, 8 -> 24);
        // Now switch to false
        p.update_branch(8, false);
        check_predict!(p, 8 -> !);
        p.update_branch(8, false);
        check_predict!(p, 8 -> !);
        // Now switch to true
        p.update_branch(8, true);
        check_predict!(p, 8 -> 24);

        // This should still BTB miss (though it aliases to 8)
        check_predict!(p, 1032 -> ());
        p.update_target(1032, 32);
        // Since it 1032 aliases with 8 and 8 is predicted taken, so is 32
        check_predict!(p, 8 -> 24, 1032 -> 32);
        p.update_branch(1032, false);
        // Updating 1032 should also update both
        check_predict!(p, 8 -> !, 1032 -> !);

        // 24 was never "update_branch"-ed; given the defaults we want it to assume not taken
        p.update_target(24, 64);
        p.update_branch(8, true); // (double-check that 8 doesn't influence 24)
        check_predict!(p, 24 -> !);
    }
}
