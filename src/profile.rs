use crate::{error::GeopropError, point::Point, tiles::Tiles};
use pyo3::{pyclass, pymethods, types::PyList, Python};
use terrain::{geo::coord, Profile as TerrainProfile};

#[pyclass]
pub(crate) struct Profile {
    pub(crate) start_alt: f32,
    pub(crate) end_alt: f32,
    pub(crate) inner: TerrainProfile<f32>,
}

impl std::ops::Deref for Profile {
    type Target = TerrainProfile<f32>;
    fn deref(&self) -> &TerrainProfile<f32> {
        &self.inner
    }
}

#[pymethods]
impl Profile {
    #[new]
    pub(crate) fn new(
        tiles: &Tiles,
        start: Point,
        end: Point,
        earth_curve: Option<bool>,
        earth_radius_m: Option<f32>,
    ) -> Result<Profile, GeopropError> {
        let max_step = 90.0;
        let start_alt = start.alt;
        let end_alt = end.alt;
        let start_coord = coord!(x: start.lon, y: start.lat);
        let end_coord = coord!(x: end.lon, y: end.lat);
        let mut builder = TerrainProfile::<f32>::builder()
            .start(start_coord)
            .start_alt(start.alt)
            .max_step(max_step)
            .end(end_coord)
            .end_alt(end.alt);

        if let Some(earth_curve) = earth_curve {
            builder = builder.earth_curve(earth_curve);
        }

        if let Some(earth_radius_m) = earth_radius_m {
            builder = builder.earth_radius(earth_radius_m);
        }

        let inner = builder.build(tiles)?;
        Ok(Profile {
            start_alt,
            end_alt,
            inner,
        })
    }

    /// Incremental path distance for all lists in the profile.
    pub(crate) fn distances<'a>(&self, py: Python<'a>) -> &'a PyList {
        PyList::new(py, self.inner.distances_m.iter())
    }

    /// Location of each step along the great circle route from
    /// `start` to `end`.
    pub(crate) fn great_circle<'a>(&self, py: Python<'a>) -> &'a PyList {
        PyList::new(
            py,
            self.inner
                .great_circle
                .iter()
                .map(|coord| (coord.y(), coord.x())),
        )
    }

    /// Terrain elevation at each step along the great circle route
    /// from `start` to `end`.
    pub(crate) fn elevation<'a>(&self, py: Python<'a>) -> &'a PyList {
        PyList::new(py, self.inner.terrain_elev_m.iter())
    }

    /// Elevation samples for a straigt line from `start` to `end`.
    pub(crate) fn los<'a>(&self, py: Python<'a>) -> &'a PyList {
        PyList::new(py, self.inner.los_elev_m.iter())
    }
}
