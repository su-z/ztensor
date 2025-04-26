use std::ops::Range;
use super::omega_int;
use dyn_clone::DynClone;

/// Type alias for finite indices used in ZTensors.
/// Uses 64-bit integers to represent finite index values.
pub type FiniteIndex = i64;

/// Type alias for omega indices used in ZTensors.
/// Represents potentially infinite indices for tensor dimensions.
pub type OmegaIndex = omega_int::OmegaInt<FiniteIndex>;

/// Trait for cloneable functions that compute tensor elements.
///
/// This trait allows functions to be stored in trait objects while remaining
/// cloneable, making it possible to copy tensors with their value getters.
pub trait CloneableFn<const N:usize, Elem>: DynClone + Fn(&[FiniteIndex; N]) -> Elem {}

/// Implementation of CloneableFn for any function matching the required signature.
impl<const N:usize, Elem, F> CloneableFn<N, Elem> for F
where
    F: DynClone + Fn(&[FiniteIndex; N])->Elem {}

dyn_clone::clone_trait_object!(<const N: usize, Elem> CloneableFn<N, Elem>);

/// Core trait for tensor-like objects with N dimensions.
///
/// This trait defines the fundamental operations for accessing elements and
/// dimension information in tensor-like objects that can have infinite ranges.
pub trait ZTensorLike<const N: usize> {
    /// The element type stored in this tensor
    type DType;
    
    /// Gets a single element at the specified indices.
    ///
    /// # Arguments
    ///
    /// * `indices` - Array of indices, one for each dimension
    ///
    /// # Returns
    ///
    /// The tensor element at the specified indices
    fn get_single_elem(&self, indices: &[FiniteIndex; N]) -> Self::DType;
    
    /// Returns the index ranges for all dimensions.
    ///
    /// # Returns
    ///
    /// Array of ranges defining the extents of each dimension
    fn get_index_ranges(&self) -> [Range<OmegaIndex>; N];
}

/// Trait for tensor-like objects that can be created from ranges and a value function.
///
/// This trait allows creating tensors by specifying the ranges for each dimension
/// and a function that computes the element values from indices.
pub trait ZTensorLikeFromRangesValues<const N:usize> : ZTensorLike<N> {
    /// Creates a new tensor with specified ranges and a function to compute values.
    ///
    /// # Arguments
    ///
    /// * `ranges` - Array of ranges for each dimension
    /// * `value_getter` - Function that computes the tensor element for given indices
    ///
    /// # Returns
    ///
    /// A new tensor with the specified configuration
    fn from_ranges_values<F: CloneableFn<N, Self::DType> + 'static>(ranges: &[Range<OmegaIndex>; N], value_getter: F) -> Self;
}

/// Converts a range from one type to another using Into trait.
///
/// # Arguments
///
/// * `r` - Range with start and end of type S
///
/// # Returns
///
/// Range with start and end converted to type T
pub fn range_into<S, T>(r: Range<S>) -> Range<T> where S: Into<T> {
    Range { start: r.start.into(), end: r.end.into() }
}

/// Trait for tensor-like objects that support slicing.
///
/// This trait allows extracting sub-tensors by specifying ranges
/// for each dimension.
pub trait ZTensorLikeSlice<const N:usize> : ZTensorLike<N> {
    /// Creates a slice of this tensor with the specified ranges.
    ///
    /// # Arguments
    ///
    /// * `ranges` - Array of ranges to slice each dimension
    ///
    /// # Returns
    ///
    /// A new tensor representing the slice
    fn get_slice(&self, ranges: &[Range<OmegaIndex>; N]) -> Self;
}

/// Trait for tensor-like objects that support slicing with generic index types.
///
/// This trait extends slicing functionality to work with any index type
/// that can be converted to OmegaIndex.
pub trait ZTensorLikeSliceGenericIndex<const N:usize> : ZTensorLikeSlice<N> + Sized {
    /// Creates a slice using ranges with generic index types.
    ///
    /// # Arguments
    ///
    /// * `ranges` - Array of ranges with indices that can be converted to OmegaIndex
    ///
    /// # Returns
    ///
    /// A new tensor representing the slice
    fn get_slice_generic(&self, ranges: &[Range<impl Into<OmegaIndex> + Clone>; N]) -> Self {
        let ranges = ranges.clone();
        let ranges: [Range<OmegaIndex>; N] = ranges.map(|r|{range_into(r)});
        self.get_slice(&ranges)
    }
}

/// Blanket implementation of ZTensorLikeSliceGenericIndex for all types that implement ZTensorLikeSlice.
impl<T, const N: usize> ZTensorLikeSliceGenericIndex<N> for T where
    T: ZTensorLikeSlice<N> + Sized {}

/// Default implementation of ZTensorLikeSlice for any tensor type that satisfies the requirements.
impl<const N: usize, TS: ZTensorLike<N> + ZTensorLikeFromRangesValues<N> + Clone + 'static> ZTensorLikeSlice<N> for TS {
    fn get_slice(&self, ranges: &[Range<OmegaIndex>; N]) -> Self {
        let self2 = (*self).clone();
        let eval_closure = move |indices: &[FiniteIndex; N]|{self2.get_single_elem(indices)};
        TS::from_ranges_values(ranges, eval_closure)
    }
}

/// Trait for tensor-like objects that can be created from ranges with generic index types.
///
/// This trait extends creation functionality to work with any index type
/// that can be converted to OmegaIndex.
pub trait ZTensorLikeFromRangesValuesGenericIndex<const N: usize, D> : ZTensorLike<N, DType = D> + Sized + ZTensorLikeFromRangesValues<N> {
    /// Creates a new tensor with ranges using generic index types.
    ///
    /// # Arguments
    ///
    /// * `ranges` - Array of ranges with indices that can be converted to OmegaIndex
    /// * `value_getter` - Function that computes the tensor element for given indices
    ///
    /// # Returns
    ///
    /// A new tensor with the specified configuration
    fn from_ranges_values_generic<F: CloneableFn<N, D> + 'static>(ranges: &[Range<impl Into<OmegaIndex> + Clone>; N], value_getter: F) -> Self {
        let ranges = ranges.clone();
        Self::from_ranges_values(&(ranges.map(|r|{range_into(r)})), value_getter)
    }
}

/// Blanket implementation of ZTensorLikeFromRangesValuesGenericIndex for all types that satisfy the requirements.
impl<const N: usize, D, T> ZTensorLikeFromRangesValuesGenericIndex<N, D> for T where 
    T: ZTensorLike<N, DType = D> + Sized + ZTensorLikeFromRangesValues<N>
{}

/// Trait for 0-dimensional tensor-like objects (scalars).
pub trait ZScalarLike : ZTensorLike<0> {}

/// Blanket implementation of ZScalarLike for all 0-dimensional tensor types.
impl<T: ZTensorLike<0>> ZScalarLike for T {}

/// Trait for 1-dimensional tensor-like objects (vectors).
pub trait ZVectorLike : ZTensorLike<1> {}

/// Blanket implementation of ZVectorLike for all 1-dimensional tensor types.
impl<T: ZTensorLike<1>> ZVectorLike for T {}

/// Trait for 2-dimensional tensor-like objects (matrices).
pub trait ZMatrixLike : ZTensorLike<2> {}

/// Blanket implementation of ZMatrixLike for all 2-dimensional tensor types.
impl<T: ZTensorLike<2>> ZMatrixLike for T {}
