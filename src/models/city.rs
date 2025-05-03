use super::coordinates::Coordinates;
use super::uf::UF;

#[derive(Debug, Clone)]
pub struct City {
    id: u16,
    uf: UF,
    coordinates: Coordinates,
}

impl City {
    pub fn new(id: u16, uf: UF, coordinates: Coordinates) -> Self {
        City {
            id,
            uf,
            coordinates,
        }
    }

    pub fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }

    pub fn get_uf(&self) -> &UF {
        &self.uf
    }
}
