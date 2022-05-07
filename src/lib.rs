#![deny(clippy::all)]
use dlib_face_recognition::*;
use image::*;
use napi::bindgen_prelude::*;
use napi_derive::napi;

extern crate napi_derive;

#[macro_use]
extern crate lazy_static;

#[napi(object)]
pub struct FaceLocation {
  pub left: u32,
  pub top: u32,
  pub right: u32,
  pub bottom: u32,
}

lazy_static! {
  static ref DETECTOR: FaceDetector = FaceDetector::default();
  static ref DETECTOR_CNN: FaceDetectorCnn = FaceDetectorCnn::default();
  static ref LANDMARKS: LandmarkPredictor = LandmarkPredictor::default();
}

#[napi]
fn face_locations(input: Buffer) -> Vec<FaceLocation> {
  let bytes = input.as_bytes();
  let image = image::load_from_memory(bytes).unwrap().to_rgb8();
  let matrix = ImageMatrix::from_image(&image);
  let locations = DETECTOR.face_locations(&matrix);

  locations.iter().map(|location| {
    FaceLocation {
      left: location.left as u32,
      top: location.top as u32,
      right: location.right as u32,
      bottom: location.bottom as u32,
    }
  }).collect()
}
