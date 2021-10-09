use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};

pub struct PngImage {
    imageBuffer: RgbImage,
    pub width: u32,
    pub height: u32,
}

impl PngImage {
    pub fn new(width: u32, height: u32) -> PngImage {
        return PngImage {
            width,
            height,
            imageBuffer: ImageBuffer::new(width, height),
        };
    }

    pub fn save(&self, path: &String) {
        self.imageBuffer.save_with_format(
            path, ImageFormat::Png,
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, data: Rgb<u8>) {
        self.imageBuffer.put_pixel(x, y, data);
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Rgb<u8> {
        return self.imageBuffer.get_pixel(x, y);
    }
}