use core::marker::PhantomData;

use crate::{Field, FieldCopyAccess, FieldReadExt, FieldWriteExt};

/// A field view represents the field metadata stored in a [Field] plus it stores the underlying
/// storage data it operates on, either as a reference to a slice `&[u8]`, `&mut [u8]`, or as
/// an owning [`Vec<u8>`].
///
/// Since this API remembers the underlying storage data in a view object, you don't have to pass it
/// in each time you're accessing a field. If you rather prefer an API that does not do that,
/// take a look at the [Field] API.
///
/// # Example:
/// ```
/// use binary_layout::prelude::*;
///
/// define_layout!(my_layout, LittleEndian, {
///   field_one: u16,
///   another_field: [u8; 16],
///   something_else: u32,
///   tail: [u8],
/// });
///
/// fn func(storage_data: &mut [u8]) {
///   let mut view = my_layout::View::new(storage_data);
///
///   // read some data
///   let format_version_header: u16 = view.field_one().read();
///   // equivalent: let format_version_header = u16::from_le_bytes((&storage_data[0..2]).try_into().unwrap());
///
///   // write some data
///   view.something_else_mut().write(10);
///   // equivalent: data_slice[18..22].copy_from_slice(&10u32.to_le_bytes());
///
///   // access a data region
///   let tail: &[u8] = view.tail();
///   // equivalent: let tail: &[u8] = &data_slice[22..];
///
///   // and modify it
///   view.tail_mut()[..5].copy_from_slice(&[1, 2, 3, 4, 5]);
///   // equivalent: data_slice[18..22].copy_from_slice(&[1, 2, 3, 4, 5]);
/// }
/// ```
pub struct FieldView<S, F: Field> {
    storage: S,
    _p: PhantomData<F>,
}

impl<S, F: Field> FieldView<S, F> {
    /// Create a new view for a field over a given storage.
    /// You probably shouldn't call this directly but should instead call
    /// `your_layout::View::new()`, which is generated by the
    /// [define_layout!](crate::define_layout!) macro for you.
    #[inline(always)]
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            _p: PhantomData,
        }
    }
}
impl<S: AsRef<[u8]>, F: FieldReadExt> FieldView<S, F> {
    /// Read the field from a given data region, assuming the defined layout, using the [FieldView] API.
    ///
    /// # Example
    /// ```
    /// use binary_layout::prelude::*;
    ///
    /// define_layout!(my_layout, LittleEndian, {
    ///   //... other fields ...
    ///   some_integer_field: i8
    ///   //... other fields ...
    /// });
    ///
    /// fn func(storage_data: &[u8]) {
    ///   let view = my_layout::View::new(storage_data);
    ///   let read: i8 = view.some_integer_field().read();
    /// }
    /// ```
    #[inline(always)]
    pub fn read(&self) -> F::HighLevelType {
        F::read(self.storage.as_ref())
    }
}
impl<S: AsMut<[u8]>, F: FieldWriteExt> FieldView<S, F> {
    /// Write the field to a given data region, assuming the defined layout, using the [FieldView] API.
    ///
    /// # Example
    /// ```
    /// use binary_layout::prelude::*;
    ///
    /// define_layout!(my_layout, LittleEndian, {
    ///   //... other fields ...
    ///   some_integer_field: i8
    ///   //... other fields ...
    /// });
    ///
    /// fn func(storage_data: &mut [u8]) {
    ///   let mut view = my_layout::View::new(storage_data);
    ///   view.some_integer_field_mut().write(10);
    /// }
    /// ```
    #[inline(always)]
    pub fn write(&mut self, v: F::HighLevelType) {
        F::write(self.storage.as_mut(), v)
    }
}
impl<S: AsRef<[u8]>, F: FieldCopyAccess> FieldView<S, F> {
    /// Read the field from a given data region, assuming the defined layout, using the [FieldView] API.
    ///
    /// # Example
    /// ```
    /// use binary_layout::prelude::*;
    /// use core::num::NonZeroI8;
    ///
    /// define_layout!(my_layout, LittleEndian, {
    ///   //... other fields ...
    ///   some_integer_field: core::num::NonZeroI8,
    ///   //... other fields ...
    /// });
    ///
    /// fn func(storage_data: &[u8]) -> Result<NonZeroI8, NonZeroIsZeroError> {
    ///   let view = my_layout::View::new(storage_data);
    ///   let read: NonZeroI8 = view.some_integer_field().try_read()?;
    ///   Ok(read)
    /// }
    /// ```
    #[inline(always)]
    pub fn try_read(&self) -> Result<F::HighLevelType, F::ReadError> {
        F::try_read(self.storage.as_ref())
    }
}
impl<S: AsMut<[u8]>, F: FieldCopyAccess> FieldView<S, F> {
    /// Write the field to a given data region, assuming the defined layout, using the [FieldView] API.
    ///
    /// # Example
    /// ```
    /// use binary_layout::prelude::*;
    /// use core::num::NonZeroI8;
    /// use core::convert::Infallible;
    ///
    /// define_layout!(my_layout, LittleEndian, {
    ///   //... other fields ...
    ///   some_integer_field: core::num::NonZeroI8,
    ///   //... other fields ...
    /// });
    ///
    /// fn func(storage_data: &mut [u8]) -> Result<(), Infallible> {
    ///   let mut view = my_layout::View::new(storage_data);
    ///   let value = NonZeroI8::new(10).unwrap();
    ///   view.some_integer_field_mut().try_write(value)?;
    ///   Ok(())
    /// }
    /// ```
    #[inline(always)]
    pub fn try_write(&mut self, v: F::HighLevelType) -> Result<(), F::WriteError> {
        F::try_write(self.storage.as_mut(), v)
    }
}
