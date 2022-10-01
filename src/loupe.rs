use crate::vector2::Vector2;

#[derive(Debug)]
pub struct Loupe {
    pub center: Vector2<f32>,
    pub zoom: f32,
}

impl Loupe {
    /// Gets the coordinates that are some percentage from the centre.
    pub fn get(&self, pc: &Vector2<f32>) -> Vector2<f32> {
        Vector2 {
            x: self.center.x + (pc.x / self.zoom),
            y: self.center.y + (pc.y / self.zoom),
        }
    }

    pub fn new() -> Loupe {
        Loupe {
            center: Vector2 { x: -0.765, y: 0.0 },
            zoom: 0.3,
        }
    }
}

impl PartialEq for Loupe {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.zoom == other.zoom
    }
}
