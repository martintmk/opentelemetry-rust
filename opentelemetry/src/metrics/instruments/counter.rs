use crate::KeyValue;
use core::fmt;
use std::rc::Rc;

use super::SyncInstrument;

/// An instrument that records increasing values.
///
/// [`Counter`] can be cloned to create multiple handles to the same instrument. If a [`Counter`] needs to be shared,
/// users are recommended to clone the [`Counter`] instead of creating duplicate [`Counter`]s for the same metric. Creating
/// duplicate [`Counter`]s for the same metric could lower SDK performance.
#[derive(Clone)]
#[non_exhaustive]
pub struct Counter<T>(Rc<dyn SyncInstrument<T>>);

impl<T> fmt::Debug for Counter<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Counter<{}>", std::any::type_name::<T>()))
    }
}

impl<T> Counter<T> {
    /// Create a new counter.
    pub fn new(inner: Rc<dyn SyncInstrument<T>>) -> Self {
        Counter(inner)
    }

    /// Records an increment to the counter.
    pub fn add(&self, value: T, attributes: &[KeyValue]) {
        self.0.measure(value, attributes)
    }
}

/// An async instrument that records increasing values.
#[derive(Clone)]
#[non_exhaustive]
pub struct ObservableCounter<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> ObservableCounter<T> {
    /// Create a new observable counter.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ObservableCounter {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> fmt::Debug for ObservableCounter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "ObservableCounter<{}>",
            std::any::type_name::<T>()
        ))
    }
}
