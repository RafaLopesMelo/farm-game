use crate::internal::world::coords::Coords2D;

pub struct Camera {
    coords: Coords2D,
    zoom: u16,
}

impl Camera {
    pub fn new(coords: Coords2D, zoom: u16) -> Result<Self, CameraError> {
        if zoom > 500 {
            return Err(CameraError::InvalidZoom(zoom));
        }

        if zoom < 25 {
            return Err(CameraError::InvalidZoom(zoom));
        }

        return Ok(Self { coords, zoom });
    }
}

#[derive(Debug)]
pub enum CameraError {
    InvalidZoom(u16),
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraError::InvalidZoom(zoom) => {
                write!(f, "Invalid zoom: {}", zoom)
            }
        }
    }
}

impl std::error::Error for CameraError {}
