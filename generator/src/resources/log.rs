use cgmath::Vector2;
use std::fs::{File, OpenOptions};

pub struct Log {
    pub file: File,
    pub line: String,
}

impl Default for Log {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Log {
    pub fn new(num_entities: usize, num_iterations: usize) -> Self {
        Self {
            file: OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!(".output/{}-{}.txt", num_entities, num_iterations))
                .unwrap(),
            line: String::with_capacity(num_entities * 2 * 64),
        }
    }

    pub fn push(&mut self, position: Vector2<f64>, _velocity: Vector2<f64>) {
        use std::fmt::Write;

        write!(&mut self.line, "{} {},", position.x, position.y).unwrap();
    }

    pub fn next(&mut self) {
        {
            use std::fmt::Write;

            write!(&mut self.line, "\n").unwrap();
        }

        {
            use std::io::Write;

            self.file.write(self.line.as_bytes()).unwrap();

            self.line.clear();
        }
    }
}
