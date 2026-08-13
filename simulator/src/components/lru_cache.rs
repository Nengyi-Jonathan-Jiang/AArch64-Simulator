use crate::zero_init::zeroInit_lenient;
use crate::zero_init::zeroInit;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use hybrid_array::{Array, ArraySize};
use crate::zero_init::ZeroInit;
use macro_rules_attribute::derive;

pub trait LRUCache<T> {
    /// Search for an entry satisfying the given predicate. If multiple entries satisfy the
    /// predicate, there is no guarantee for which one is returned. The returned entry is promoted
    /// to be the most recently used entry.
    fn get(&self, cond: impl FnMut(&T) -> bool) -> Option<&T>;
    fn get_mut(&mut self, cond: impl FnMut(&T) -> bool) -> Option<&mut T>;

    /// Get a mutable reference to the least recently used entry and promotes it to the most
    /// recently used entry.
    fn get_lru(&mut self) -> &mut T;

    /// [Search for an entry satisfying the given predicate](LRUCache::get). If no entry is found,
    /// `insert` is called to initialize a new entry (replacing the [lru entry](LRUCache::get_lru)),
    /// which is returned instead.
    fn get_or_insert(
        &mut self,
        cond: impl FnMut(&T) -> bool,
        mut insert: impl FnMut(&mut T),
    ) -> &mut T {
        let x: Option<*mut T> = self.get_mut(cond).map(|x| x as _);

        if x.is_some() {
            return unsafe { x.unwrap().as_mut_unchecked() };
        }

        let res = self.get_lru();
        insert(res);
        res
    }
}

// It makes sense to allow zero init'ing when data is logically zero-init-able
#[derive(zeroInit_lenient!)]
pub struct FixedSizeLRUCache<T, N: ArraySize> {
    // Use raw UnsafeCell; we want to be as efficient as possible
    state: UnsafeCell<MatrixLRUState<N>>,
    data: Array<T, N>,
}

impl<T, N: ArraySize> FixedSizeLRUCache<T, N> {
    fn update(&self, i: usize) {
        unsafe {
            self.state.as_mut_unchecked().update(i);
        }
    }
}

impl<T, N: ArraySize> LRUCache<T> for FixedSizeLRUCache<T, N> {
    fn get(&self, mut cond: impl FnMut(&T) -> bool) -> Option<&T> {
        for i in 0..N::USIZE {
            if cond(&self.data[i]) {
                self.update(i);
                return Some(&self.data[i]);
            }
        }

        None
    }

    fn get_mut(&mut self, mut cond: impl FnMut(&T) -> bool) -> Option<&mut T> {
        for i in 0..N::USIZE {
            if cond(&self.data[i]) {
                self.update(i);
                return Some(&mut self.data[i]);
            }
        }

        None
    }

    fn get_lru(&mut self) -> &mut T {
        let lru = self.state.get_mut().get_lru();
        let res = &mut self.data[lru];
        self.state.get_mut().update(lru);
        res
    }
}

impl<T, N: ArraySize> FixedSizeLRUCache<T, N> {
    fn new(mut initial_data: impl FnMut() -> T) -> Self {
        Self {
            state: Default::default(),
            data: Array::from_fn(|_| initial_data()),
        }
    }
}

impl<T: Default, N: ArraySize> Default for FixedSizeLRUCache<T, N> {
    fn default() -> Self {
        Self::new(|| Default::default())
    }
}

// TODO: extend MatrixLRUState to handle N > 8, preferably up to N = 32

/// A data structure that tracks the least recently used element out of a fixed set of `N` elements
/// encoded as integers from `0` to `N - 1`
#[derive(zeroInit!)]
struct MatrixLRUState<N: ArraySize> {
    /// Invariant: when interpreted as an 8 x 8 matrix of bits in row major order and considering
    /// only the top left N x N submatrix, the bit at row r, col c is set iff element c was accessed
    /// more recently than element r
    matrix: u64,
    _n: PhantomData<N>,
}

impl<N: ArraySize> Default for MatrixLRUState<N> {
    fn default() -> Self {
        Self {
            // The zero matrix trivially satisfies the invariant -- it represents the case where no
            // element was accessed more recently than any other element -- and is the natural
            // default, by the same reasoning
            matrix: 0,
            _n: PhantomData,
        }
    }
}

impl<N: ArraySize> MatrixLRUState<N> {
    /// Access the given element (`0 ≤ access < N`) and return the new least recently used element
    fn update(&mut self, n: usize) {
        debug_assert!(n < N::USIZE);

        // Set the nth column (this element was accessed more recently than all elements)
        self.matrix |= 0x0101010101010101u64 << n;
        // Unset the nth row (no element was accessed more recently than this element)
        self.matrix &= !(0xffu64 << (n * 8));
    }

    fn get_lru(&self) -> usize {
        /// Get a 64-bit mask for the first `n` bits, where `1 ≤ n ≤ 64`. Values outside this range
        /// are wrapped (as in modular arithmetic).
        fn get_mask(n: usize) -> u64 {
            // Put a 1 in the `n`th bit (one-indexed)
            let p = 0x_8000_0000_0000_0000_u64.rotate_left(n as u32);
            // Spread to all lower bits
            p | (p - 1)
        }

        // Generate masks to extract submatrix
        let row_mask: u64 = get_mask(N::USIZE * 8);
        let col_mask: u64 = get_mask(N::USIZE);

        // We want to find a fully unset column (whose element was not accessed more recently than
        // any element), which will be an LRU element.
        //
        // Due to the invariant, this will always exist. Since "used more recently than" is a strict
        // partial order and the set of elements we are concered with is finite, there exists at
        // least one element that was not "used more recently than" every other element. (This
        // follows from the fact that every finite poset has at least one maximal element)
        let mut matrix = self.matrix;

        // Unset all but the first N rows
        matrix &= row_mask;
        // Bitor all rows to extract the target columns that has all 0's
        matrix |= matrix >> 32;
        matrix |= matrix >> 16;
        matrix |= matrix >> 8;
        // Unset all but the first N bits
        matrix &= col_mask;
        // The number of trailing ones is the index of the first zero column
        matrix.trailing_ones() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{U1, U2, U5, U6, U8};

    macro_rules! state_update {
        ($cache: ident <- $($access: expr),+) => {
            $($cache.update($access);)+
        };
    }

    macro_rules! check_state_update {
        (($cache: ident <- $access: literal) == $expected: literal) => {
            $cache.update($access);
            assert_eq!($cache.get_lru(), $expected);
        };
    }

    #[test]
    fn test_lru_state_matrix_1() {
        let mut state = MatrixLRUState::<U1>::default();
        check_state_update!((state <- 0) == 0);
        check_state_update!((state <- 0) == 0);
        check_state_update!((state <- 0) == 0);
        check_state_update!((state <- 0) == 0);
    }

    #[test]
    fn test_lru_state_matrix_2() {
        let mut state = MatrixLRUState::<U2>::default();
        check_state_update!((state <- 0) == 1);
        check_state_update!((state <- 0) == 1);
        check_state_update!((state <- 1) == 0);
        check_state_update!((state <- 1) == 0);
        check_state_update!((state <- 0) == 1);
    }

    #[test]
    fn test_lru_state_matrix_8() {
        let mut state = MatrixLRUState::<U8>::default();

        //                      v-- LRU        MRU --v
        state_update!(state <- 0, 5, 3, 6, 2, 7, 1, 4);

        check_state_update!((state <- 4) == 0); // Does not change order
        check_state_update!((state <- 0) == 5); // 5, 3, 6, 2, 7, 1, 4, 0
        check_state_update!((state <- 4) == 5); // 5, 3, 6, 2, 7, 1, 0, 4
        check_state_update!((state <- 6) == 5); // 5, 3, 2, 7, 1, 0, 4, 6

        state_update!(state <- 0, 5, 3); // 2, 7, 1, 4, 6, 0, 5, 3

        check_state_update!((state <- 1) == 2); // 2, 7, 4, 6, 0, 5, 3, 1
        check_state_update!((state <- 2) == 7); // 7, 4, 6, 0, 5, 3, 1, 2
        check_state_update!((state <- 7) == 4); // 4, 6, 0, 5, 3, 1, 2, 7
    }

    #[test]
    fn test_lru_state_matrix_5() {
        let mut state = MatrixLRUState::<U5>::default();

        state_update!(state <- 0, 3, 2, 1, 4);

        check_state_update!((state <- 4) == 0); // Does not change order
        check_state_update!((state <- 0) == 3); // 3, 2, 1, 4, 0
        check_state_update!((state <- 4) == 3); // 3, 2, 1, 0, 4
        check_state_update!((state <- 0) == 3); // 3, 2, 1, 4, 0
        check_state_update!((state <- 2) == 3); // 3, 1, 4, 0, 2
        check_state_update!((state <- 3) == 1); // 1, 4, 0, 2, 3
        check_state_update!((state <- 1) == 4); // 4, 0, 2, 3, 1
    }

    macro_rules! check_cache_get {
        (($target:literal in $cache: ident) == $expected: literal) => {
            assert_eq!($cache.get(|x| x & 0xf == $target).cloned(), Some($expected));
        };
        (($target:literal in $cache: ident) == $expected: ident) => {
            assert_eq!($cache.get(|x| x & 0xf == $target), $expected);
        };
    }

    #[test]
    fn test_lru_cache() {
        // Cache is initialized with zeros
        let mut c = FixedSizeLRUCache::<u8, U6>::default();
        let cache = &mut c;
        
        check_cache_get!((0x0 in cache) == 0);
        check_cache_get!((0xe in cache) == None);
        check_cache_get!((0x5 in cache) == None);

        *cache.get_lru() = 0xb5;

        check_cache_get!((0x0 in cache) == 0);
        check_cache_get!((0xe in cache) == None);
        check_cache_get!((0x5 in cache) == 0xb5);

        *cache.get_lru() = 0xce;

        check_cache_get!((0x0 in cache) == 0);
        check_cache_get!((0xe in cache) == 0xce);
        check_cache_get!((0x5 in cache) == 0xb5);

        //                       v--LRU          MRU--v
        // Now cache should have 0, 0, 0, 0, 0xce, 0xb5

        for _ in 0..4 {
            // Access the four LRU elements (0)
            assert_eq!(*cache.get_lru(), 0);
        }

        // Now cache should have 0xce, 0xb5, 0, 0, 0, 0

        let evicted = cache.get_lru();
        assert_eq!(*evicted, 0xce); // This should return the spot where 0xce is stored
        *evicted = 0x2f; // Replace 0xce with 0x2f

        // Now cache should have 0xb5, 0, 0, 0, 0x2f

        check_cache_get!((0xe in cache) == None);
        check_cache_get!((0x5 in cache) == 0xb5);
        check_cache_get!((0xf in cache) == 0x2f);

        // By renewing access to 0xb5 and 0x2f above, we should have put them back near the front
        // Cache should have 0, 0, 0, 0, 0xb5, 0x2f

        for _ in 0..4 {
            assert_eq!(*cache.get_lru(), 0);
        }
        assert_eq!(*cache.get_lru(), 0xb5);
        assert_eq!(*cache.get_lru(), 0x2f);
    }
}
