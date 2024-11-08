use embed::{EmbedImage, EmbedText};
use image::DynamicImage;

pub mod clip_image_processor;
pub mod config;
pub mod embed;

pub struct EncoderState {
    pub embed_text: EmbedText,
    pub embed_image: EmbedImage,
}

pub fn get_clip_image_tensor(dyn_img: DynamicImage) -> Vec<f32> {
    let embed_image = EmbedImage::new("models/clip/image/model.onnx").unwrap();
    embed_image.encode(dyn_img.clone()).unwrap()
}

pub fn get_clip_text_tensor(text: String) -> Vec<f32> {
    let embed_text = EmbedText::new(
        "models/clip/image/model.onnx",
        "sentence-transformers/clip-ViT-B-32-multilingual-v1",
    )
    .unwrap();
    embed_text.encode(&text).unwrap()
}
