use image::GenericImageView;
use std::env;


fn main() {
  let cwd = env::current_dir().unwrap();
  println!("The current directory is {}", cwd.as_os_str().to_str().unwrap().to_string());
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.

    img_to_text("test/01.PNG");

}

fn img_to_text(path: &str) {

  let mut img = image::open(path).unwrap();

  img.thumbnail(1700,1700);

  // The dimensions method returns the images width and height.
  println!("dimensions {:?}", img.dimensions());

  // The color method returns the image's `ColorType`.
  println!("{:?}", img.color());
  let width=img.width();
  let height = img.height();

  let img2 = image::imageops::crop(&mut img,40,40,width-80,height-80);

  // The dimensions method returns the images width and height.
  println!("crop dimensions {:?}", img2.dimensions());


  // Write the contents of this image to the Writer in PNG format.
  //img.save("test/result.png").unwrap();

  //img2.to_image().save("test/result.png").unwrap();
}