#![allow(clippy::cast_sign_loss)]

use image::RgbImage;

pub struct FrameSaver {
    index: i32,
}

impl FrameSaver {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn save_frame(&mut self, data: &[u8], w: i32, h: i32) -> bool {
        let img = RgbImage::from_raw(w as u32, h as u32, data.to_vec())
            .expect("container should have the right size for the image dimensions");

        let extension = "png";
        let file_name = format!("frame{:04}.{}", self.index, &extension);

        self.index += 1;

        match img.save(&file_name) {
            Ok(()) => true,
            Err(error) => {
                eprintln!(
                    "Cannot save frame as image to file {}. Error: {}",
                    &file_name, error
                );
                false
            }
        }
    }
}
