//! Generic form controls.

use std::borrow::Cow;

mod field;
mod select;

pub use field::{
    FormAddons, FormAddonsProps, FormField, FormFieldContainer, FormFieldContainerProps,
    FormFieldProps, FormGroup, FormGroupProps,
};
pub use select::{DataSelect, DataSelectProps};

/// An interface for the data entries.
pub trait DataEntry {
    /// Returns the unique key.
    fn key(&self) -> Cow<'_, str>;

    /// Returns the value.
    fn value(&self) -> Cow<'_, str>;

    /// Returns the label.
    fn label(&self) -> Cow<'_, str>;
}

impl<'a> DataEntry for [&'a str; 2] {
    #[inline]
    fn key(&self) -> Cow<'_, str> {
        self[0].into()
    }

    #[inline]
    fn value(&self) -> Cow<'_, str> {
        self[0].into()
    }

    #[inline]
    fn label(&self) -> Cow<'_, str> {
        self[1].into()
    }
}

impl<T: ToString, U: ToString> DataEntry for (T, U) {
    #[inline]
    fn key(&self) -> Cow<'_, str> {
        self.0.to_string().into()
    }

    #[inline]
    fn value(&self) -> Cow<'_, str> {
        self.0.to_string().into()
    }

    #[inline]
    fn label(&self) -> Cow<'_, str> {
        self.1.to_string().into()
    }
}
