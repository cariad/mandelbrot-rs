#[cfg(test)]
mod tests {
    use mandelbrot_rs::loupe::Loupe;
    use mandelbrot_rs::vector2::Vector2;

    const TOP_LEFT: Vector2<f32> = Vector2 { x: -1.0, y: -1.0 };
    const TOP_RIGHT: Vector2<f32> = Vector2 { x: 1.0, y: -1.0 };
    const BOTTOM_LEFT: Vector2<f32> = Vector2 { x: -1.0, y: 1.0 };
    const BOTTOM_RIGHT: Vector2<f32> = Vector2 { x: 1.0, y: 1.0 };

    #[test]
    fn new() {
        let expect = Loupe {
            center: Vector2 { x: -0.765, y: 0.0 },
            zoom: 0.3,
        };
        assert_eq!(Loupe::new(), expect);
    }

    #[test]
    fn get_top_left() {
        assert_eq!(
            Loupe::new().get(&TOP_LEFT),
            Vector2 {
                x: -4.0983334,
                y: -3.3333333
            }
        );
    }

    #[test]
    fn get_top_right() {
        assert_eq!(
            Loupe::new().get(&TOP_RIGHT),
            Vector2 {
                x: 2.5683331,
                y: -3.3333333
            }
        );
    }

    #[test]
    fn get_bottom_left() {
        assert_eq!(
            Loupe::new().get(&BOTTOM_LEFT),
            Vector2 {
                x: -4.0983334,
                y: 3.3333333
            }
        );
    }

    #[test]
    fn get_bottom_right() {
        assert_eq!(
            Loupe::new().get(&BOTTOM_RIGHT),
            Vector2 {
                x: 2.5683331,
                y: 3.3333333
            }
        );
    }
}
