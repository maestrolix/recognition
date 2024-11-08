pub mod arc_face_image;
pub mod postprocessor;
pub mod predictor;

use crate::ml::arcface::predictor::ArcFacePredictor;
use ort::OrtError;
use postprocessor::ArcFaceOutput;
use std::path::Path;

pub fn get_arcface_tensor(img_path: &str) -> Result<ArcFaceOutput, OrtError> {
    let arc_face_model_path = Path::new("models/arcface/resnet100.onnx");

    let arcface_predictor = ArcFacePredictor::new(arc_face_model_path)?;

    let raw_image = image::open(img_path).unwrap();

    arcface_predictor.run(&raw_image)
}
