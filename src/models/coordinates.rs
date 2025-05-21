#[derive(Debug, Clone)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}

impl Coordinates {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Coordinates {
            latitude,
            longitude,
        }
    }

    pub fn get_latitude(&self) -> f32 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f32 {
        self.longitude
    }

    pub fn get_as_array(&self) -> [f32; 2] {
        [self.latitude, self.longitude]
    }
}
