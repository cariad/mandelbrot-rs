#[cfg(test)]
mod tests {
    use mandelbrot_rs::vector2::Vector2;

    #[test]
    fn display() {
        let a: Vector2<u8> = Vector2 { x: 3, y: 7 };
        assert_eq!(a.to_string(), "(3, 7)");
    }

    #[test]
    fn eq() {
        let a: Vector2<u8> = Vector2 { x: 3, y: 7 };
        let b: Vector2<u8> = Vector2 { x: 3, y: 7 };
        assert_eq!(a, b)
    }

    #[test]
    fn ne() {
        let a: Vector2<u8> = Vector2 { x: 3, y: 7 };
        let b: Vector2<u8> = Vector2 { x: 4, y: 7 };
        assert_ne!(a, b)
    }
}
