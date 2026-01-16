pub(super) struct Node {
    pub id: i64,
    pub coord: Coord,
}

pub(super) struct RoadRaw {
    pub id: i64,
    pub nodes: Vec<i64>,
    pub kind: String,
    pub width: f32,
    pub speedlimit: u8,
}

#[derive(Default)]
pub struct Road {
    /// (min, max)
    pub extent: (Coord, Coord),
    pub points: Vec<Coord>,
    /// In meters
    pub width: f32,
    pub speedlimit: u8,
}

pub enum ObjectKind {
    Generic,
    Grass,
    Water,
    Building,
}

pub struct Object {
    pub kind: ObjectKind,
    pub shape: Vec<Coord>,
}

pub struct Map {
    pub roads: Vec<Road>,
    pub objects: Vec<Object>,
    /// (min, max)
    pub extent: (Coord, Coord),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

impl Coord {
    pub fn min_each(self, other: Self) -> Self {
        Self {
            lat: self.lat.min(other.lat),
            lon: self.lon.min(other.lon),
        }
    }

    pub fn max_each(self, other: Self) -> Self {
        Self {
            lat: self.lat.max(other.lat),
            lon: self.lon.max(other.lon),
        }
    }

    /// Returns the distance between 2 coordinates in meters
    pub fn distance_to(&self, other: Self) -> f64 {
        const R: f64 = 6371e3;

        let lat1 = self.lat.max(other.lat);
        let lat2 = self.lat.min(other.lat);

        let lon1 = self.lon.max(other.lon);
        let lon2 = self.lon.min(other.lon);

        let phi1 = lat1 * std::f64::consts::PI / 180f64;
        let phi2 = lat2 * std::f64::consts::PI / 180f64;
        let delta_phi = (lat2 - lat1) * std::f64::consts::PI / 180f64;
        let delta_lambda = (lon2 - lon1) * std::f64::consts::PI / 180f64;

        let half_sin_delta_phi = (delta_phi / 2f64).sin();
        let half_sin_delta_lambda = (delta_lambda / 2f64).sin();

        let a = half_sin_delta_phi * half_sin_delta_phi
            + phi1.cos() * phi2.cos() * half_sin_delta_lambda * half_sin_delta_lambda;
        let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());

        R * c
    }
}
