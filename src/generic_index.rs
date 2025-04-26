use std::ops::{Deref, DerefMut};

/// A generic indexing trait that allows for more flexible index types and returns.
///
/// This trait extends the functionality of Rust's standard indexing traits by allowing
/// the indexed value to be wrapped in a custom derefable type. This enables additional
/// behaviors when accessing elements through indexing syntax.
pub trait Index<Idx> {
    /// The type of the element being returned from indexing.
    type Output;
    
    /// The type of the wrapper that derefs to Output.
    ///
    /// This associated type should implement Deref<Target = Self::Output> and is
    /// what actually gets returned from the index operation.
    type DerefOutput<'a>: Deref<Target = Self::Output> where Self:'a;
    
    /// Accesses an element at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index used to access the element
    ///
    /// # Returns
    ///
    /// A wrapper type that derefs to the indexed element
    fn index<'a>(&'a self, index: Idx) -> Self::DerefOutput<'a>;
}

/// A generic mutable indexing trait that extends the Index trait.
///
/// This trait allows for mutable access to elements through indexing syntax,
/// with the indexed value wrapped in a custom type that implements DerefMut.
pub trait IndexMut<Idx>: Index<Idx> {
    /// The type of the wrapper that derefs mutably to Output.
    ///
    /// This associated type should implement DerefMut<Target = Self::Output> and is
    /// what gets returned from the mutable index operation.
    type DerefMutOutput<'a>: DerefMut<Target = Self::Output> where Self:'a;
    
    /// Mutably accesses an element at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index used to access the element
    ///
    /// # Returns
    ///
    /// A wrapper type that derefs mutably to the indexed element
    fn index_mut<'a>(&'a mut self, index: Idx) -> Self::DerefMutOutput<'a>;
}
