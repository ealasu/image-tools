use tempfile::NamedTempFile;
use pos::Pos;

pub struct Aligner {
}

impl Aligner {
    pub fn new() -> Self {
        Aligner {}
    }

    pub fn align(&mut self, image: &NamedTempFile) -> Pos {
        Pos { x: 0.0, y: 0.0 }
    }
}
