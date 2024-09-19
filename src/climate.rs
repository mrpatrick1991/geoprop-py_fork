use pyo3::pyclass;

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
