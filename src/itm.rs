use crate::{
    climate::Climate, error::GeopropError, mdvar::ModeVariability, point::Point,
    polarization::Polarization, profile::Profile, tiles::Tiles,
};
use h3o::Resolution;
use pyo3::{pyclass, pymethods};
use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator};
use terrain::{geo::Coord, Profile as TerrainProfile};

const SQRT_3: f64 = 1.732_050_8_f64;

#[pyclass]
pub(crate) struct Itm {
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
impl Itm {
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
    ) -> Itm {
        Itm {
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

    /// Retuns signal loss between two points.
    pub(crate) fn p2p(&self, profile: &Profile, freq_hz: f64) -> Result<f64, GeopropError> {
        let attenuation_db = itm::p2p(
            profile.start_alt.into(),
            profile.end_alt.into(),
            profile.distances_m[1].into(),
            &profile.terrain_elev_m,
            self.climate.into(),
            self.n0,
            freq_hz,
            self.pol.into(),
            self.epsilon,
            self.sigma,
            self.mdvar.into(),
            self.time,
            self.location,
            self.situation,
        )?;
        Ok(attenuation_db)
    }

    /// Returns signal losses between two points.
    pub(crate) fn path(&self, profile: &Profile, freq_hz: f64) -> Result<Vec<f64>, GeopropError> {
        let terrain = &profile.terrain_elev_m;
        let step_size_m = profile.distances_m[1];

        let loss_path_db = (1..terrain.len())
            .into_par_iter()
            .map(|end_idx| {
                let terrain = &terrain[..=end_idx];
                itm::p2p(
                    profile.start_alt.into(),
                    profile.end_alt.into(),
                    step_size_m.into(),
                    terrain,
                    self.climate.into(),
                    self.n0,
                    freq_hz,
                    self.pol.into(),
                    self.epsilon,
                    self.sigma,
                    self.mdvar.into(),
                    self.time,
                    self.location,
                    self.situation,
                )
            })
            .collect::<Result<Vec<f64>, _>>()?;
        Ok(loss_path_db)
    }

    pub(crate) fn coverage(
        &self,
        center: Point,
        res: u8,
        freq_hz: f64,
        max_radius_km: f64,
        rx_alt_m: f64,
        rx_threshold_db: Option<f64>,
    ) -> Result<Vec<(u64, f32, f64)>, GeopropError> {
        let res = Resolution::try_from(res).unwrap();
        let ll = h3o::LatLng::new(center.lat as f64, center.lon as f64).unwrap();
        let cell = ll.to_cell(res);
        let mut hexes = Vec::new();
        let edge_length = res.edge_length_km();
        let start_coord = Coord {
            x: center.lon,
            y: center.lat,
        };

        for ring in (0..).take_while(|ring| *ring as f64 <= max_radius_km / (edge_length * SQRT_3))
        {
            let cells = cell.grid_ring_fast(ring).collect::<Option<Vec<_>>>();

            hexes.par_extend(cells.into_par_iter().flatten().map(|cell| {
                let latlng = h3o::LatLng::from(cell);

                let profile_res = TerrainProfile::<f32>::builder()
                    .start(start_coord)
                    .start_alt(center.alt)
                    .max_step(90.0)
                    .end(Coord {
                        x: latlng.lng() as f32,
                        y: latlng.lat() as f32,
                    })
                    .end_alt(rx_alt_m as f32)
                    .build(&self.tiles);

                match profile_res {
                    Err(e) => Err(GeopropError::from(e)),
                    Ok(profile) if profile.distances_m.len() <= 1 => Ok((
                        u64::from(cell),
                        *profile.terrain_elev_m.last().unwrap(),
                        0.0,
                    )),
                    Ok(profile) => {
                        let step_size_m = profile.distances_m[1];
                        let terrain = profile.terrain_elev_m;
                        itm::p2p(
                            center.alt.into(),
                            rx_alt_m,
                            step_size_m.into(),
                            &terrain,
                            self.climate.into(),
                            self.n0,
                            freq_hz,
                            self.pol.into(),
                            self.epsilon,
                            self.sigma,
                            self.mdvar.into(),
                            self.time,
                            self.location,
                            self.situation,
                        )
                        .map_err(GeopropError::from)
                        .map(|loss| (u64::from(cell), *terrain.last().unwrap(), loss))
                    }
                }
            }));
        }

        hexes
            .into_iter()
            .filter(|res| {
                rx_threshold_db
                    .map(|rxt| match res {
                        Err(_) => true,
                        Ok((_cell, _elev, atten)) => *atten < rxt,
                    })
                    .unwrap_or(true)
            })
            .collect::<Result<Vec<_>, _>>()
    }
}
