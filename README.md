# ZTensor

ZTensor is a Rust library for working with tensors (multi-dimensional arrays) that can have potentially infinite dimensions, using omega integers for indexing.  In the name, "Z" stands for $\mathbb Z.$

This library provides a flexible framework for mathematical operations on tensors with unique features.

## Features

- **Infinite Dimensions**: Support for potentially infinite tensor dimensions using omega integers (ω)
- **Lazy Evaluation**: Elements are computed on-demand through value functions
- **Complex Numbers**: Native support for complex number elements
- **Slicing**: Flexible slicing operations on any dimension
- **Linear Algebra**: Integration with nalgebra for matrix operations
- **Generic Indexing**: Extended indexing capabilities beyond Rust's standard traits

## Core Components

### Omega Integers

ZTensor uses extended integer types that can represent infinity:

- `OmegaInt` - Integers extended with positive and negative infinity
- `OmegaUInt` - Unsigned integers extended with infinity

### Tensor Types

- `ZTensor<N>` - Generic N-dimensional tensor
- `ZScalar` - 0-dimensional tensor (scalar)
- `ZVector` - 1-dimensional tensor (vector)
- `ZMatrix` - 2-dimensional tensor (matrix)

### Trait System

The library provides a comprehensive trait system:

- `ZTensorLike` - Core trait for tensor-like objects
- `ZTensorLikeFromRangesValues` - For tensors created from ranges and value functions
- `ZTensorLikeSlice` - For tensors that support slicing
- `ToNAlgMat` - For converting tensors to nalgebra matrices

## Usage Examples

### Creating a Tensor

```rust
use ztensor::ztensor_impls::*;
use ztensor::ztensor_traits::*;
use ztensor::omega_int::OmegaInt;
use num_complex::Complex;

// Create a 2D matrix with specified ranges and a function to compute values
let matrix: ZMatrix = ZMatrix::from_ranges_values(
    &[OmegaIndex::Integer(0)..OmegaIndex::Integer(5), 
      OmegaIndex::Integer(0)..OmegaIndex::Integer(4)], 
    |[i, j]| Complex::<f32>::new(i as f32, j as f32)
);

// Access an element
let element = matrix[[2, 3]];
assert_eq!(*element, Complex::<f32>::new(2.0, 3.0));
```

### Working with Infinite Dimensions

```rust
use ztensor::ztensor_impls::*;
use ztensor::ztensor_traits::*;
use ztensor::omega_int::OmegaInt::*;

// Create a matrix with infinite dimensions
let infinite_matrix: ZMatrix = ZMatrix::from_ranges_values(
    &[MOmega..POmega, MOmega..POmega], 
    |[i, j]| (i + j) as f32 * 0.5
);

// Extract a finite slice
let finite_slice = infinite_matrix.get_slice(
    &[Integer(-10)..Integer(10), Integer(-5)..Integer(5)]
);
```

### Converting to nalgebra Matrix

```rust
use ztensor::to_nalg_mat::*;
use ztensor::ztensor_traits::*;

// Convert ZMatrix to nalgebra DMatrix
let nalg_mat = matrix.to_nalg_mat();

// Perform linear algebra operations with nalgebra
let result = nalg_mat * nalg_mat.transpose();

// Convert back to ZMatrix
let result_zmat = nalgebra_mat_to_zmat(result);
```

## Mathematical Foundation

ZTensor is built on the concept of extended number systems that include infinity, allowing for representations of mathematical objects that traditionally require special handling, such as:

- Infinite matrices and tensors
- Operators on infinite-dimensional spaces
- Formal power series
- Convolution operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ztensor = "0.1.0"
```

## License

This project is licensed under GPL-3.0 - see the LICENSE file for details. This license (GNU General Public License v3 (GPL-3.0)) applies only to the original code in this repository and not to its dependencies.
