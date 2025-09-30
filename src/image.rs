use std::{
    fs::File,
    io::{BufWriter, Result, Write},
    path::Path,
};

use crate::color::Color;

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32, data: Vec<Color>) -> Option<Self> {
        if width * height != data.len() as u32 {
            return None;
        }

        Some(Self {
            width,
            height,
            data,
        })
    }

    pub fn write_p3<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3\n{} {}\n255", self.width, self.height)?;

        for pixel in &self.data {
            pixel.write(&mut writer)?;
        }

        writer.flush()
    }
}
