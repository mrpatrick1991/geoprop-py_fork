use pyo3::pyclass;

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
