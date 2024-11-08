pub mod post_processor;
pub mod ultra_image;
pub mod ultra_predictor;

use crate::services::facial_recognition::FaceError;
use post_processor::UltraOutput;

use crate::ml::ultraface::ultra_image::UltraImage;
use crate::ml::ultraface::ultra_predictor::UltraPredictor;
use std::path::Path;
use std::process;

pub fn get_ultraface_tensor(img_path: &str) -> Result<UltraOutput, FaceError> {
    let ultra_model_path = Path::new("models/ultraface/version-RFB-640.onnx");

    let ultra_predictor = UltraPredictor::new(ultra_model_path).unwrap_or_else(|ort_err| {
        println!("Problem creating ultra onnx session: {}", ort_err);
        process::exit(1)
    });
    let mut ultra_image = UltraImage::new(Path::new(img_path))?;

    let ultraface_output = ultra_predictor.run(&ultra_image.image)?;
    additional_logic_for_debug(&mut ultra_image, &ultraface_output);

    Ok(ultraface_output)
}

pub fn additional_logic_for_debug(ultra_image: &mut UltraImage, ultra_output: &UltraOutput) {
    ultra_image
        .draw_bboxes(
            ultra_output.bbox_with_confidences.clone(),
            Path::new("tests/"),
        )
        .unwrap();
}
