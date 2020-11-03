pub const CANVAS_WIDTH: i64 = 480;
pub const CANVAS_HEIGHT: i64 = 480;
pub const TILE_WIDTH: i64 = 32;
pub const TILE_HEIGHT: i64 = 32;
pub const CANVAS_WIDTH_IN_TILE: i64 = 15;
pub const CANVAS_HEIGHT_IN_TILE: i64 = 15;
pub const HALF_CANVAS_WIDTH_IN_TILE: i64 = 7;
pub const HALF_CANVAS_HEIGHT_IN_TILE: i64 = 7;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(CANVAS_WIDTH, TILE_WIDTH * CANVAS_WIDTH_IN_TILE);
        assert_eq!(CANVAS_HEIGHT, TILE_HEIGHT * CANVAS_HEIGHT_IN_TILE);
        assert_eq!(CANVAS_WIDTH_IN_TILE, HALF_CANVAS_WIDTH_IN_TILE * 2 + 1);
        assert_eq!(CANVAS_HEIGHT_IN_TILE, HALF_CANVAS_HEIGHT_IN_TILE * 2 + 1);
    }
}
