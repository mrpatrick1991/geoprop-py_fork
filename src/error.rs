use itm::ItmErrCode;
use pyo3::{exceptions::PyValueError, PyErr};
use terrain::TerrainError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeopropError {
    #[error("{0}")]
    Terrain(#[from] TerrainError),
    #[error("{0}")]
    Itm(#[from] ItmErrCode),
}

impl From<GeopropError> for PyErr {
    fn from(other: GeopropError) -> PyErr {
        PyValueError::new_err(other.to_string())
    }
}
