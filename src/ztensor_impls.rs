use super::ztensor_traits::*;
use super::generic_index::Index;
use std::ops::{Deref, Range};
use num_complex::Complex;

/// Element type used in ZTensor implementations.
/// Uses complex numbers with 32-bit floating point components.
pub type Elem = Complex<f32>;

/// A tensor implementation supporting dimensions indexed with omega integers.
///
/// ZTensor is a generic N-dimensional tensor that can have potentially infinite
/// ranges, using omega integers for indexing. The actual values are computed
/// on-demand through a function.
#[derive(Clone)]
pub struct ZTensor<const N: usize> {
    /// The index ranges for each dimension
    index_ranges: [Range<OmegaIndex>; N],
    /// Function that computes the tensor elements given indices
    value_getter: Box<dyn CloneableFn<N, Elem>>
}

/// Reference to a ZTensor element.
///
/// This wrapper provides dereferencing capabilities to access the underlying
/// complex value.
pub struct ZTensorElemRef {
    /// The actual element value
    value: Elem
}

impl Deref for ZTensorElemRef {
    type Target = Elem;
    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

/// Implementation of ZTensorLike trait for ZTensor.
///
/// This provides the core functionality for accessing tensor elements and ranges.
impl<const N: usize> ZTensorLike<N> for ZTensor<N> {
    type DType = Elem;
    
    /// Returns the index ranges for all dimensions.
    fn get_index_ranges(&self) -> [Range<OmegaIndex>; N] {
        return self.index_ranges.clone();
    }
    
    /// Gets a single element at the specified indices.
    fn get_single_elem(&self, indices: &[FiniteIndex; N]) -> Self::DType {
        return (self.value_getter)(indices);
    }
}

/// Implementation for creating ZTensor from ranges and a value function.
impl<const N: usize> ZTensorLikeFromRangesValues<N> for ZTensor<N> {
    /// Creates a new ZTensor with specified ranges and a function to compute values.
    ///
    /// # Arguments
    ///
    /// * `ranges` - Array of ranges for each dimension
    /// * `value_getter` - Function that computes the tensor element for given indices
    fn from_ranges_values<F: CloneableFn<N, Self::DType> + 'static>(ranges: &[Range<OmegaIndex>; N], value_getter: F) -> Self {
        let bo: Box<dyn CloneableFn<N, Elem>> = Box::new(value_getter);
        Self {index_ranges: ranges.clone(), value_getter: bo}
    }
}

/// Implementation of Index trait for ZTensor.
///
/// Allows using array indexing syntax `(tensor[indices])` to access elements.
impl<const N: usize> Index<[FiniteIndex; N]> for ZTensor<N> {
    type Output = Elem;
    type DerefOutput<'a> = ZTensorElemRef;
    
    /// Returns a reference to the element at the specified indices.
    fn index<'a>(&'a self, index: [FiniteIndex; N]) -> Self::DerefOutput<'a> {
        ZTensorElemRef {value: self.get_single_elem(&index)}
    }
}

#[test]
fn test_ztensor(){
    use super::omega_int::OmegaInt;
    use num_traits::ToPrimitive;
    use OmegaInt::*;
    fn value_getter(index: &[FiniteIndex; 2]) -> Elem {
        return (index[0] + index[1]*10).to_f32().unwrap().into();
    }

    let t = ZTensor::<2>::from_ranges_values(&[Integer(0)..Integer(3), Integer(0)..Integer(4)], value_getter);
    assert_eq!(t.get_single_elem(&[2,3]), 32.0.into());
    assert_eq!(*t.index([1,2]), 21.0.into());

    // Test truncations
    let t2 = t.get_slice(&[Integer(0)..Integer(3), Integer(0)..Integer(3)]);
    assert_eq!(*t2.index([1,2]), 21.0.into());
}

/// Type alias for a 0-dimensional ZTensor (scalar).
pub type ZScalar = ZTensor<0>;

/// Type alias for a 1-dimensional ZTensor (vector).
pub type ZVector = ZTensor<1>;

/// Type alias for a 2-dimensional ZTensor (matrix).
pub type ZMatrix = ZTensor<2>;

impl ZMatrix {
    /// Returns the conjugate transpose of this matrix.
    ///
    /// This method creates a new matrix by swapping dimensions and taking
    /// the complex conjugate of each element.
    ///
    /// # Returns
    ///
    /// A new ZMatrix representing the conjugate transpose
    pub fn conj_trans(&self) -> Self {
        let valget = self.value_getter.clone();
        ZMatrix::from_ranges_values(&[self.index_ranges[1].clone(), self.index_ranges[0].clone()], move|&[i, j]|{
            let val = valget(&[j, i]);
            val.conj()
        })
    }
}
