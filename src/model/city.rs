
use super::coordinates::Coordinates;

#[derive(Debug)]
pub struct City {
    id: u16,
    coordinates: Coordinates
}

impl City {
    pub fn new(id: u16, coordinates: Coordinates) -> Self {
        City { id, coordinates }
    }

    pub fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}