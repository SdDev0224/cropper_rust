use image::GenericImageView;
use image::imageops::crop;


fn crop_image(image_path: &str, left_top: (u32, u32), right_bottom: (u32, u32), output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load the image
    let img = image::open(image_path)?;
    
    // Calculate width and height from coordinates
    let width = right_bottom.0 - left_top.0;
    let height = right_bottom.1 - left_top.1;

    // Validate coordinates
    let (img_width, img_height) = img.dimensions();
    if right_bottom.0 > img_width || right_bottom.1 > img_height {
        return Err("Crop coordinates exceed image dimensions".into());
    }
}