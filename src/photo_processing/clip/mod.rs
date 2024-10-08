use embed::{EmbedImage, EmbedText};

pub mod clip_image_processor;
pub mod config;
pub mod embed;

pub struct EncoderState {
    pub embed_text: EmbedText,
    pub embed_image: EmbedImage,
}
