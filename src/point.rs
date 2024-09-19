use pyo3::{pyclass, pymethods};
use terrain::geo::Coord;

#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Point {
    /// Degrees northing.
    #[pyo3(get, set)]
    pub lat: f32,

    /// Degrees easting.
    #[pyo3(get, set)]
    pub lon: f32,

    /// Meters above an abstract, application-specific, referece
    /// elevation.
    #[pyo3(get, set)]
    pub alt: f32,
}

#[pymethods]
impl Point {
    #[new]
    pub(crate) fn new(lat: f32, lon: f32, alt: Option<f32>) -> Point {
        Point {
            lat,
            lon,
            alt: alt.unwrap_or_default(),
        }
    }

    fn __repr__(&self) -> String {
        format!("Point({}, {}, {})", self.lat, self.lon, self.alt)
    }
}

impl Point {
    pub(crate) fn into_coord(self) -> Coord<f64> {
        Coord {
            y: self.lat as f64,
            x: self.lon as f64,
        }
    }
}
