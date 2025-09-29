use std::{
    fs::File,
    io::{BufWriter, Result, Write},
    path::Path,
};

use crate::color::Color;

pub struct Image {}

impl Image {
    pub fn write<P, F>(path: P, width: u32, height: u32, color: F) -> Result<()>
    where
        P: AsRef<Path>,
        F: Fn(u32, u32) -> Color,
    {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        write!(writer, "P3\n{} {}\n255\n", width, height)?;

        for j in 0..height {
            for i in 0..width {
                color(i, j).write(&mut writer)?;
            }
        }

        writer.flush()
    }
}
