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

    /// Counts the iterations at the coordinates that are some percentage from
    /// the centre.
    pub fn iterations(&self, pc: &Vector2<f32>, max: u16) -> u16 {
        let coords = self.get(&pc);

        let mut count: u16 = 0;
        let mut t = Vector2 { x: 0.0, y: 0.0 };
        let mut t_squared = Vector2 { x: 0.0, y: 0.0 };

        while count < max && t_squared.x + t_squared.y <= 4.0 {
            t_squared.x = f32::powi(t.x, 2);
            t_squared.y = f32::powi(t.y, 2);

            t.y = 2.0 * t.x * t.y + coords.y;
            t.x = t_squared.x - t_squared.y + coords.x;

            count += 1;
        }

        count
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
