#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
mod py;
#[cfg(feature = "serde")]
mod serde;

pub mod buscal;

pub use buscal::{BusdayConvention, BusinessCalendar};

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule]
fn rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BusinessCalendar>()?;
    m.add_class::<BusdayConvention>()?;
    Ok(())
}
