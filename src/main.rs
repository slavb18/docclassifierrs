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

  let img = image::open(path).unwrap();

  img.thumbnail(1700,1700);

  // The dimensions method returns the images width and height.
  println!("dimensions {:?}", img.dimensions());

  // The color method returns the image's `ColorType`.
  println!("{:?}", img.color());

  // Write the contents of this image to the Writer in PNG format.
  img.save("test/result.png").unwrap();
}