#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
            if rank > 7 || rank < 0 || file < 0 ||file > 7 {
                  return None;
            }
            Some(ChessPosition{
                  rank: rank,
                  file: file,
            })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
            Queen{
                  position: ChessPosition::new(position.rank, position.file).unwrap()
            }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
            self.position.rank == other.position.rank || self.position.file == other.position.file ||
            ((self.position.rank - other.position.rank) as f64).abs() == ((self.position.file - other.position.file) as f64).abs()  
    }
}