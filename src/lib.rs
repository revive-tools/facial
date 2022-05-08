#![deny(clippy::all)]
use dlib_face_recognition::*;
use image::*;
use napi::bindgen_prelude::*;
use napi_derive::napi;

extern crate napi_derive;

#[macro_use]
extern crate lazy_static;

#[napi(object)]
pub struct Location {
  pub left: u32,
  pub top: u32,
  pub right: u32,
  pub bottom: u32,
}

fn to_rectangle(
  Location {
    left,
    top,
    right,
    bottom,
  }: &Location,
) -> dlib_face_recognition::Rectangle {
  dlib_face_recognition::Rectangle {
    left: *left as i64,
    top: *top as i64,
    right: *right as i64,
    bottom: *bottom as i64,
  }
}

fn to_location(
  Rectangle {
    left,
    top,
    right,
    bottom,
  }: &Rectangle,
) -> Location {
  Location {
    left: *left as u32,
    top: *top as u32,
    right: *right as u32,
    bottom: *bottom as u32,
  }
}

#[napi(object)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

fn to_point(point: dlib_face_recognition::Point) -> Point {
  Point {
    x: point.x() as u32,
    y: point.y() as u32,
  }
}

lazy_static! {
  static ref DETECTOR: FaceDetector = FaceDetector::default();
  static ref LANDMARKS: LandmarkPredictor = LandmarkPredictor::default();
  static ref ENCODER: FaceEncoderNetwork = FaceEncoderNetwork::default();
}

struct Facial {
  pub matrix: ImageMatrix,
}

impl Facial {
  pub fn new(matrix: ImageMatrix) -> Self {
    Facial { matrix }
  }
}

#[napi(js_name = "Facial")]
struct JsFacial {
  facial: Facial,
}

#[napi]
impl JsFacial {
  #[napi(factory)]
  pub fn from_image(input: Buffer) -> Self {
    let bytes = input.as_bytes();
    let image = image::load_from_memory(bytes).unwrap().to_rgb8();
    let matrix = ImageMatrix::from_image(&image);
    JsFacial {
      facial: Facial::new(matrix),
    }
  }

  #[napi]
  pub fn locations(&mut self) -> Vec<Location> {
    let locations = DETECTOR.face_locations(&self.facial.matrix);

    locations
      .iter()
      .map(|location| to_location(&*location))
      .collect()
  }

  #[napi]
  pub fn landmarks(&mut self, locations: Vec<Location>) -> Vec<Vec<Point>> {
    locations
      .iter()
      .map(|location| to_rectangle(location))
      .map(|rectangle| LANDMARKS.face_landmarks(&self.facial.matrix, &rectangle))
      .map(|landmarks| {
        let mut points = Vec::new();
        landmarks
          .iter()
          .map(|point| to_point(*point))
          .for_each(|point| points.push(point));
        return points;
      })
      .collect()
  }

  #[napi]
  pub fn encodings(&mut self, location: Location, jitters: Option<u32>) -> Vec<f64> {
    let num_jitters = jitters.unwrap_or(0);
    let rect = to_rectangle(&location);
    let landmarks = LANDMARKS.face_landmarks(&self.facial.matrix, &rect);
    let encodings = ENCODER.get_face_encodings(&self.facial.matrix, &[landmarks], num_jitters);
    let encoding = encodings.get(0).expect("No face encoding found");

    Vec::from(encoding.as_ref())
  }
}

#[napi]
pub fn compare_faces(one: Vec<f64>, two: Vec<f64>) -> f64 {
  let first_encoding = FaceEncoding::from_vec(one).expect("Invalid encodings provided for first face");
  let second_encoding = FaceEncoding::from_vec(two).expect("Invalid encodings provided for second face");
  
  first_encoding.distance(&second_encoding)
}
