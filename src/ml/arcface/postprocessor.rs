use ort::{tensor::OrtOwnedTensor, OrtError, Value};

pub type Bbox = [f32; 4];
pub type UltraResult = Vec<(Bbox, f32)>;

pub struct UltraOutput {
    pub bbox_with_confidences: UltraResult,
}

pub struct ArcFaceOutput {
    pub embedding: Vec<f32>,
}

impl ArcFaceOutput {
    pub fn new(outputs: Vec<Value>) -> Result<ArcFaceOutput, OrtError> {
        let output_1: OrtOwnedTensor<f32, _> = outputs[0].try_extract()?;
        let embeddings_view = output_1.view();
        let embeddings_arr = embeddings_view.to_slice().unwrap().to_vec();
        Ok(ArcFaceOutput {
            embedding: embeddings_arr,
        })
    }
}
