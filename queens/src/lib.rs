#[derive(Debug)]
pub struct ChessPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Option<Self> {
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            return Some(ChessPosition { x, y });
        } else {
            return None;
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.x == other.position.x ||
            self.position.y == other.position.y ||
            (self.position.x - other.position.x).abs() == (self.position.y - other.position.y).abs()
    }
}
