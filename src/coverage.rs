use crate::{error::GeopropError, point::Point, tiles::Tiles};
use h3o::Resolution;
use pyo3::{pyclass, pymethods};
use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator};
use terrain::{geo::Coord, Profile};

const SQRT_3: f32 = 1.732_050_8_f32;

/// Given a transmitter at `center`, estimate its coverage taking
/// terrain into account.
///
/// # Example
///
/** ```python

from geoprop import Tiles, Point, Coverage

tiles = Tiles("nasadem/3-arcsecond/srtm/")
center = Point(36.159600, -112.306877, 1000)
rx_alt_m = 1
h3_res = 10
freq_hz = 900e6
radius_km = 12

coverage = Coverage(tiles)
estimated_coverage = coverage.estimate(center, h3_res, freq_hz, radius_km, rx_alt_m, rx_threshold_db = None)

print("h3_id,elev,atten")
for (cell, elev, atten) in estimated_coverage:
    print("%x,%d,%f" % (cell, elev, -atten))

``` */
#[pyclass]
pub(crate) struct Coverage {
    tiles: Tiles,

    #[pyo3(get, set)]
    climate: Climate,

    #[pyo3(get, set)]
    n0: f64,

    #[pyo3(get, set)]
    pol: Polarization,

    #[pyo3(get, set)]
    epsilon: f64,

    #[pyo3(get, set)]
    sigma: f64,

    #[pyo3(get, set)]
    mdvar: ModeVariability,

    #[pyo3(get, set)]
    time: f64,

    #[pyo3(get, set)]
    location: f64,

    #[pyo3(get, set)]
    situation: f64,
}

#[pymethods]
impl Coverage {
    #[new]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        tiles: &Tiles,
        climate: Option<Climate>,       //  = Climate::ContinentalTemperate;
        n0: Option<f64>,                //  = 301.0;
        pol: Option<Polarization>,      //  = Polarization::Vertical;
        epsilon: Option<f64>,           //  = 15.0;
        sigma: Option<f64>,             //  = 0.005;
        mdvar: Option<ModeVariability>, //  = ModeVariability::Mobile;
        time: Option<f64>,              //  = 95.0;
        location: Option<f64>,          //  = 95.0;
        situation: Option<f64>,         //  = 95.0;
    ) -> Coverage {
        Coverage {
            tiles: tiles.clone(),
            climate: climate.unwrap_or(Climate::ContinentalTemperate),
            n0: n0.unwrap_or(301.0),
            pol: pol.unwrap_or(Polarization::Vertical),
            epsilon: epsilon.unwrap_or(15.0),
            sigma: sigma.unwrap_or(0.005),
            mdvar: mdvar.unwrap_or(ModeVariability::Mobile),
            time: time.unwrap_or(95.0),
            location: location.unwrap_or(95.0),
            situation: situation.unwrap_or(95.0),
        }
    }

}
#[derive(Copy, Clone, Debug)]
#[pyclass]
pub(crate) enum Climate {
    Equatorial = 1,
    ContinentalSubtropical = 2,
    MaritimeSubtropical = 3,
    Desert = 4,
    ContinentalTemperate = 5,
    MaritimeTemperateOverLand = 6,
    MaritimeTemperateOverSea = 7,
}

impl From<Climate> for itm::Climate {
    fn from(other: Climate) -> Self {
        match other {
            Climate::Equatorial => itm::Climate::Equatorial,
            Climate::ContinentalSubtropical => itm::Climate::ContinentalSubtropical,
            Climate::MaritimeSubtropical => itm::Climate::MaritimeSubtropical,
            Climate::Desert => itm::Climate::Desert,
            Climate::ContinentalTemperate => itm::Climate::ContinentalTemperate,
            Climate::MaritimeTemperateOverLand => itm::Climate::MaritimeTemperateOverLand,
            Climate::MaritimeTemperateOverSea => itm::Climate::MaritimeTemperateOverSea,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[pyclass]
pub(crate) enum ModeVariability {
    SingleMessage = 0,
    Accidental = 1,
    Mobile = 2,
    Broadcast = 3,
}

impl From<ModeVariability> for itm::ModeVariability {
    fn from(other: ModeVariability) -> Self {
        match other {
            ModeVariability::SingleMessage => itm::ModeVariability::SingleMessage,
            ModeVariability::Accidental => itm::ModeVariability::Accidental,
            ModeVariability::Mobile => itm::ModeVariability::Mobile,
            ModeVariability::Broadcast => itm::ModeVariability::Broadcast,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[pyclass]
pub(crate) enum Polarization {
    Horizontal = 0,
    Vertical = 1,
}

impl From<Polarization> for itm::Polarization {
    fn from(other: Polarization) -> Self {
        match other {
            Polarization::Horizontal => itm::Polarization::Horizontal,
            Polarization::Vertical => itm::Polarization::Vertical,
        }
    }
}
