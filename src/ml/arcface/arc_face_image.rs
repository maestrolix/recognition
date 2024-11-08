use image::{imageops::FilterType, DynamicImage, ImageError, RgbImage};

pub struct ArcFaceImage {
    pub image: RgbImage,
}

impl ArcFaceImage {
    pub fn new(raw_image: DynamicImage) -> Result<ArcFaceImage, ImageError> {
        let raw_image = DynamicImage::from(raw_image)
            .resize_to_fill(128, 128, FilterType::Triangle)
            .to_rgb8();

        return Ok(ArcFaceImage { image: raw_image });
    }
}
