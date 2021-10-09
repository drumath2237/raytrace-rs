use image::{ImageBuffer, ImageFormat, ImageResult, Rgb, RgbImage};

#[derive(Debug)]
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

    pub fn save(&self, path: &str) -> ImageResult<()> {
        self.imageBuffer.save_with_format(path, ImageFormat::Png)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, data: Rgb<u8>) {
        self.imageBuffer.put_pixel(x, y, data);
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Rgb<u8> {
        return self.imageBuffer.get_pixel(x, y);
    }
}

#[cfg(test)]
mod png_image_test {
    use image::Rgb;
    use crate::png_image::PngImage;

    #[test]
    fn new_test() {
        let image = PngImage::new(64, 64);

        assert_eq!(image.height, 64);
        assert_eq!(image.width, 64);
        assert_eq!(image.imageBuffer.width(), 64);
        assert_eq!(image.imageBuffer.height(), 64);
    }

    #[test]
    fn save_test() {
        let mut img = PngImage::new(16, 16);

        for y in 0..img.height {
            for x in 0..img.width {
                img.set_pixel(x, y, Rgb([255, 0, 255]));
            }
        }

        match std::fs::metadata("./images") {
            Err(_) => {
                std::fs::create_dir("./images");
            }
            _ => {}
        }

        let result = img.save("./images/image.png");
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn get_set_pixel_test() {
        let mut img = PngImage::new(16, 16);

        img.set_pixel(1, 1, Rgb([255, 0, 255]));
        assert_eq!(img.get_pixel(1, 1)[0], 255 as u8);

        img.set_pixel(2, 2, Rgb([0, 0, 255]));
        assert_eq!(img.get_pixel(2, 2)[0], 0 as u8);
    }
}