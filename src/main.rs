use image::GenericImageView;
use image::DynamicImage;
use std::env;
use imageproc::contrast::adaptive_threshold;

// use tesseract::ocr;
use tesseract::Tesseract;

fn main() {
  let cwd = env::current_dir().unwrap();
  println!("The current directory is {}", cwd.as_os_str().to_str().unwrap().to_string());

  img_to_text("test/01.PNG");

}

fn img_to_text(path: &str) {
  // Use the open function to load an image from a Path.
  // `open` returns a `DynamicImage` on success.
  let mut img = image::open(path).unwrap();

  img.thumbnail(1700,1700);

  // The dimensions method returns the images width and height.
  println!("dimensions {:?}", img.dimensions());

  // The color method returns the image's `ColorType`.
  println!("{:?}", img.color());
  let width=img.width();
  let height = img.height();

  let img2 = image::imageops::crop(&mut img,40,40,width-80,height-80).to_image();
  println!("crop dimensions {:?}", img2.dimensions());

  let imgg = DynamicImage::ImageRgba8(img2).into_luma8();
  // The color method returns the image's `ColorType`.
  // println!("{:?}", gray.color());

//  = note: expected reference `&image::buffer_::ImageBuffer<image::color::Luma<u8>, Vec<u8>>`
//  found reference `&ImageBuffer<Luma<u8>, Vec<u8>>`
// = note: perhaps two different versions of crate `image` are being used?
  let img3=adaptive_threshold(&imgg,10);


  // Write the contents of this image to the Writer in PNG format.
  //img.save("test/result.png").unwrap();

  img3.save("test/result.png").unwrap();

  let tesseract = Tesseract::new(None, Some("rus")).unwrap();

  let imgbuf=img3.as_raw();
  let text = tesseract
  // .set_image("test/01.PNG").unwrap()
  .set_image_from_mem(&imgbuf).unwrap()
  .recognize().unwrap()
  .get_text().unwrap();

  // let text = ocr("test/01.PNG", "rus").unwrap();
   println!("ocr =  {}", text);

}