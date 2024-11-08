use std::{path::Path, process, time::Instant};

use image::{DynamicImage, RgbImage};
use ndarray::{Array4, CowArray, IxDyn};
use ort::{
    Environment, ExecutionProvider, GraphOptimizationLevel, LoggingLevel, OrtError, Session,
    SessionBuilder, Value,
};

use crate::ml::arcface::{arc_face_image::ArcFaceImage, postprocessor::ArcFaceOutput};

pub struct ArcFacePredictor {
    pub name: String,
    pub session: Session,
}

pub static ARC_FACE_NAME: &str = "ArcFacePredictor";

impl ArcFacePredictor {
    pub fn new(model_filepath: &Path) -> Result<ArcFacePredictor, OrtError> {
        let start = Instant::now();

        let environment = Environment::builder()
            .with_name(ARC_FACE_NAME.to_string())
            .with_execution_providers([ExecutionProvider::CPU(Default::default())])
            .with_log_level(LoggingLevel::Verbose)
            .build()?
            .into_arc();

        let session = SessionBuilder::new(&environment)?
            .with_optimization_level(GraphOptimizationLevel::Disable)?
            .with_model_from_file(&model_filepath)?;

        println!("{} startup took {:?}", ARC_FACE_NAME, start.elapsed());
        Ok(ArcFacePredictor {
            name: ARC_FACE_NAME.to_string(),
            session,
        })
    }

    pub fn run(&self, raw_image: &DynamicImage) -> Result<ArcFaceOutput, OrtError> {
        let image = ArcFaceImage::new(raw_image.clone()).expect("something went wrong");
        let image_tensor = self.get_image_tensor(&image.image);
        let image_input = self.get_image_input(&image_tensor)?;
        let raw_outputs = self.session.run(image_input).unwrap_or_else(|err| {
            println!("somehting went wrong running session: {}", err);
            process::exit(1)
        });

        Ok(ArcFaceOutput::new(raw_outputs)?)
    }

    fn get_image_tensor(&self, image: &RgbImage) -> CowArray<f32, IxDyn> {
        let image_tensor = CowArray::from(Array4::from_shape_fn(
            (1, 3, 112 as usize, 112 as usize),
            |(_, c, y, x)| ((image[(x as _, y as _)][c] as f32 / 255.0) - 0.5) / 0.5,
        ))
        .into_dyn();

        return image_tensor;
    }

    fn get_image_input<'a>(
        &self,
        image_tensor: &'a CowArray<'a, f32, IxDyn>,
    ) -> Result<Vec<Value<'a>>, OrtError> {
        let input_value = Value::from_array(self.session.allocator(), &image_tensor)?;
        let input = vec![input_value];

        return Ok(input);
    }
}
