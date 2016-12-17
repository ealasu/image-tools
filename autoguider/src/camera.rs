use tempfile::NamedTempFile;

pub struct Camera {
}

impl Camera {
    pub fn new() -> Self {
        Camera {}
    }

    pub fn shoot(&self) -> NamedTempFile {
        let f = NamedTempFile::new().unwrap();

        f
    }
}
