mod climate;
mod error;
mod itm;
mod mdvar;
mod point;
mod polarization;
mod profile;
mod tiles;

use crate::{
    climate::Climate, itm::Itm, mdvar::ModeVariability, point::Point, polarization::Polarization,
    profile::Profile, tiles::Tiles,
};
use pyo3::{pymodule, types::PyModule, PyResult, Python};

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "geoprop")]
fn python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Climate>()?;
    m.add_class::<Itm>()?;
    m.add_class::<ModeVariability>()?;
    m.add_class::<Point>()?;
    m.add_class::<Polarization>()?;
    m.add_class::<Profile>()?;
    m.add_class::<Tiles>()?;
    Ok(())
}
