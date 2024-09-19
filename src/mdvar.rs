use pyo3::pyclass;

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
