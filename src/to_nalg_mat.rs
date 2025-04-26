use std::fmt::Debug;
use std::ops::Range;

use super::omega_int::OmegaInt;
use super::ztensor_impls::{Elem, ZMatrix};
use super::ztensor_traits::*;
use nalgebra::DMatrix;

/// Trait for converting ZTensor objects to nalgebra matrices.
///
/// This trait provides functionality to convert ZTensor objects with
/// two dimensions into nalgebra's DMatrix type for linear algebra operations.
pub trait ToNAlgMat {
    /// The element type of the resulting matrix
    type Elem;
    
    /// Converts the tensor to a nalgebra DMatrix.
    ///
    /// This method extracts the elements from a two-dimensional ZTensor
    /// and creates a corresponding nalgebra matrix with the same elements.
    fn to_nalg_mat(&self) -> DMatrix<Self::Elem>  where Self::Elem: 'static;
}

/// Implementation of ToNAlgMat for any 2D ZTensorLike type.
///
/// This allows any 2D tensor-like object to be converted to a nalgebra matrix
/// as long as its element type can be cloned and compared.
impl<T> ToNAlgMat for T where T: ZTensorLike<2>, T::DType: Clone + PartialEq + Debug{
    type Elem = T::DType;
    fn to_nalg_mat(&self) -> DMatrix<Self::Elem> where T::DType: 'static {
        let ranges = self.get_index_ranges();
        let ranges_len = ranges.clone().map(|r: Range<OmegaIndex>|{r.end - r.start});
        // Check the length of ranges are all finite
        let finite_len = ranges_len.map(|l|{
            match l {
                OmegaInt::Integer(x) => {
                    if x < 0 {panic!()}
                    x as usize
                },
                _ => panic!()
            }
        });
        let start_indices = ranges.map(|r|{
            match r.start {
                OmegaInt::Integer(x) => x,
                _ => panic!()
            }
        });
        let mat: DMatrix<Self::Elem> = DMatrix::from_fn(finite_len[0], finite_len[1], |i: usize, j: usize|{
            self.get_single_elem(&[start_indices[0]+i as FiniteIndex,start_indices[1]+j as FiniteIndex])
        });
        mat
    }
}

#[cfg(test)]
mod test {
    use num_complex::Complex;
    use num_traits::ToPrimitive;

    use super::{super::ztensor_impls::*, OmegaIndex, ToNAlgMat, ZTensorLikeFromRangesValues, ZTensorLikeSlice};

    #[test]
    fn test_ztensor_to_nalgebra_matrix(){
        let t: ZMatrix = ZMatrix::from_ranges_values(&[OmegaIndex::MOmega.. OmegaIndex::POmega, OmegaIndex::MOmega.. OmegaIndex::POmega], |[i1, i2]|{
            Complex::<f32>::new(i1.to_f32().unwrap(), i2.to_f32().unwrap())
        });
        let trunc = t.get_slice(&[OmegaIndex::Integer(-4)..OmegaIndex::Integer(4), OmegaIndex::Integer(-5)..OmegaIndex::Integer(8)]);
        let m = trunc.to_nalg_mat();
        assert_eq!(m[(2,3)], Complex::<f32>::new(-2.to_f32().unwrap(), -2.to_f32().unwrap()))
    }
}

/// Converts a nalgebra DMatrix to a ZMatrix.
///
/// This function takes a nalgebra matrix and creates a corresponding ZMatrix
/// with the same dimensions and elements. The resulting ZMatrix has finite
/// ranges starting from 0 matching the input matrix's dimensions.
///
/// # Arguments
///
/// * `mat` - The nalgebra DMatrix to convert
///
/// # Returns
///
/// A ZMatrix representation of the input matrix
pub fn nalgebra_mat_to_zmat(mat: DMatrix<Elem>) -> ZMatrix {
    ZMatrix::from_ranges_values(&[OmegaIndex::Integer(0)..OmegaIndex::Integer(mat.nrows() as FiniteIndex), OmegaIndex::Integer(0)..OmegaIndex::Integer(mat.ncols() as FiniteIndex)], move |[i1, i2]|{
        mat[(*i1 as usize,*i2 as usize)]
    })
}

#[test]
fn test_nalgebra_mat_to_zmat(){
    let mat: DMatrix<Elem> = DMatrix::from_row_slice(2,3, &([1., 2., 3., 4., 5., 6.].map(|x|{x.into()})));
    let zmat = nalgebra_mat_to_zmat(mat.clone());
    let mat2 = zmat.to_nalg_mat();
    assert_eq!(mat, mat2);
}
