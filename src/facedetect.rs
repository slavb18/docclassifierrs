
use opencv::{
  Result,
  prelude::*,
  objdetect,
  imgproc,
  core,
  types,
  imgcodecs
};

pub fn facedetect(path: &str)->Result<bool>{
  // TEST FACE DETECT
  let xml = "/usr/share/opencv4/haarcascades/haarcascade_frontalface_default.xml";
  let mut face_detector = objdetect::CascadeClassifier::new(xml)?;
  let img = imgcodecs::imread(path, -1).unwrap(); // Issue over here
  let mut gray = Mat::default();
  imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
  let mut faces = types::VectorOfRect::new();

  face_detector.detect_multi_scale(
      &gray,
      &mut faces,
      1.1,
      10,
      objdetect::CASCADE_SCALE_IMAGE,
      core::Size::new(10, 10),
      core::Size::new(0, 0)
  )?;
  println!("{:?}", faces);
  Ok(!faces.is_empty())
}
