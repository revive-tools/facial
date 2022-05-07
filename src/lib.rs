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

fn to_rectangle(FaceLocation { left, top, right, bottom }: FaceLocation) -> dlib_face_recognition::Rectangle {
  dlib_face_recognition::Rectangle {
    left: left as i64,
    top: top as i64,
    right: right as i64,
    bottom: bottom as i64,
  }
}

fn to_location(Rectangle { left, top, right, bottom }: Rectangle) -> FaceLocation {
  FaceLocation {
    left: left as u32,
    top: top as u32,
    right: right as u32,
    bottom: bottom as u32,
  }
}

#[napi(object)]
pub struct FacePoint {
  pub x: u32,
  pub y: u32,
}

fn to_face_point(point: dlib_face_recognition::Point) -> FacePoint {
  FacePoint { x: point.x() as u32, y: point.y() as u32 }
}

lazy_static! {
  static ref DETECTOR: FaceDetectorCnn = FaceDetectorCnn::default();
  static ref LANDMARKS: LandmarkPredictor = LandmarkPredictor::default();
  static ref ENCODER: FaceEncoderNetwork = FaceEncoderNetwork::default();
}


#[napi]
fn face_locations(input: Buffer) -> Vec<FaceLocation> {
  let bytes = input.as_bytes();
  let image = image::load_from_memory(bytes).unwrap().to_rgb8();
  let matrix = ImageMatrix::from_image(&image);
  let locations = DETECTOR.face_locations(&matrix);

  locations.iter().map(|location| {
    to_location(*location)
  }).collect()
}


#[napi]
fn face_landmarks(input: Buffer, location: FaceLocation) -> Vec<FacePoint> {
  let bytes = input.as_bytes();
  let image = image::load_from_memory(bytes).unwrap().to_rgb8();
  let matrix = ImageMatrix::from_image(&image);
  let rect = to_rectangle(location);
  let landmarks = LANDMARKS.face_landmarks(&matrix, &rect);
  
  landmarks.iter().map(|point| {
    to_face_point(*point)
  }).collect()
}

#[napi]
fn face_encodings(input: Buffer, location: FaceLocation, jitters: Option<u32>) -> Vec<f64> {
  let num_jitters = jitters.unwrap_or(1);
  let bytes = input.as_bytes();
  let image = image::load_from_memory(bytes).unwrap().to_rgb8();
  let matrix = ImageMatrix::from_image(&image);
  let rect = to_rectangle(location);
  let landmarks = LANDMARKS.face_landmarks(&matrix, &rect);
  
  let encodings = ENCODER.get_face_encodings(&matrix, &[landmarks], num_jitters);
  let encoding = encodings.get(0).expect("No face encoding found");
  
  Vec::from(encoding.as_ref())
}