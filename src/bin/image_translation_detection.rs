use image::{DynamicImage, GenericImageView, Pixel, RgbaImage, imageops::FilterType};
use std::path::Path;


// fn f_a_n_u8_image_diff()
// ->{

fn overlay_and_subtract_with_translation(
    img1: &DynamicImage,
    img2: &DynamicImage,
    translate_x: i32,
    translate_y: i32
) -> (RgbaImage, f64, f64, f64) {
    let (width, height) = img1.dimensions();
    let mut result_image = RgbaImage::new(width, height);
    let mut n_sum = 0.0;
    let mut n_count = 0.0;
    let mut n_cross_sum = 0.0;
    for y in 0..height {
        for x in 0..width {
            let translated_x = x as i32 + translate_x;
            let translated_y = y as i32 + translate_y;

            if translated_x >= 0 && translated_x < width as i32 && translated_y >= 0 && translated_y < height as i32 {
                let pixel1 = img1.get_pixel(x, y).to_rgba();
                let pixel2 = img2.get_pixel(translated_x as u32, translated_y as u32).to_rgba();

                // Check if both pixels are non-black
                if (pixel1[0] != 0 || pixel1[1] != 0 || pixel1[2] != 0 || pixel1[3] != 0) &&
                    (pixel2[0] != 0 || pixel2[1] != 0 || pixel2[2] != 0 || pixel2[3] != 0) {
                    // Subtract pixel values
                    let diff_pixel = image::Rgba([
                        pixel1[0].abs_diff(pixel2[0]),
                        pixel1[1].abs_diff(pixel2[1]),
                        pixel1[2].abs_diff(pixel2[2]),
                        255,
                    ]);
                    n_count+=1.0;
                    n_sum+= (diff_pixel[0] as f64 /255.+diff_pixel[1] as f64 /255.+diff_pixel[2] as f64 /255.)/3.;
                    result_image.put_pixel(x, y, diff_pixel);
                    n_cross_sum+= pixel1[0] as f64 * pixel2[0] as f64;
                }
            }
        }
    }
    let n_avg = n_sum/n_count;
    let n_avg_cross_sum = n_cross_sum/n_count;

    (result_image, n_avg, n_count, n_avg_cross_sum)
}
fn overlay_images_with_translation(
    img1: &DynamicImage,
    img2: &DynamicImage,
    translate_x: i32,
    translate_y: i32,
) -> RgbaImage {
    let (width1, height1) = img1.dimensions();
    let (width2, height2) = img2.dimensions();

    // Calculate new dimensions
    let new_width = (width1 as i32 + translate_x.max(0) + width2 as i32 - translate_x.min(0)).max(width1 as i32) as u32;
    let new_height = (height1 as i32 + translate_y.max(0) + height2 as i32 - translate_y.min(0)).max(height1 as i32) as u32;

    let mut combined_image = RgbaImage::new(new_width, new_height);

    // Draw the first image at the correct offset
    for y in 0..height1 {
        for x in 0..width1 {
            let pixel1 = img1.get_pixel(x, y).to_rgba();
            combined_image.put_pixel(x, y, pixel1);
        }
    }

    // Draw the second image with translation
    for y in 0..height2 {
        for x in 0..width2 {
            let translated_x = x as i32 + translate_x;
            let translated_y = y as i32 + translate_y;

            if translated_x >= 0 && translated_x < new_width as i32 && translated_y >= 0 && translated_y < new_height as i32 {
                let pixel2 = img2.get_pixel(x, y).to_rgba();
                combined_image.put_pixel(translated_x as u32, translated_y as u32, pixel2);
            }
        }
    }

    combined_image
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path1 = "./image_stitching/1.png";
    let path2 = "./image_stitching/2.png";

    // Load the image from the given path
    let img1 = image::open(path1)?;
    let img2 = image::open(path2)?;

    // Resize the image using the specified dimensions
    let resized_img1 = img1.resize(
        100,
         100,
          FilterType::CatmullRom);
    let resized_img2 = img2.resize(
            100,
             100,
              FilterType::CatmullRom);
    // Convert the resized image to RGBA8 format and extract the raw pixel data
    // let raw_pixels = resized_img.to_rgba8().into_raw();
    
    // resized_img.save("outtest.png")?;
    let n_x_start = -(resized_img2.width() as i32);
    let n_x_end = resized_img1.width() as i32;
    let n_y_start = -(resized_img2.height() as i32);
    let n_y_end = resized_img1.height() as i32;

    let mut n_x_best = 0;
    let mut n_y_best = 0;
    let mut n_avg_best = -1.;
    let mut n_avg_cross_sum_best = 0.;
    for n_x in n_x_start..n_x_end{

        for n_y in n_y_start..n_y_end{

            let (o, n_avg, n_count,n_avg_cross_sum) = overlay_and_subtract_with_translation(
                &resized_img1,
                &resized_img2, 
                n_x, 
                n_y
            );
            if(n_count > 50.){
                // if(n_avg_best == -1. || n_avg < n_avg_best){
                if(n_avg_cross_sum > n_avg_cross_sum_best){
                    n_x_best = n_x;
                    n_y_best = n_y;
                    n_avg_cross_sum_best = n_avg_cross_sum;
                }
            }
            // o.save( format!("out_diff_{}_{}.png", n_x, n_y));
        }
    }
    println!("n avg{}, n_x {}, n_y {}, best", n_avg_best, n_x_best, n_y_best);


    let mut ores = overlay_images_with_translation(
        &resized_img2,
        &resized_img1,
        n_x_best,
        n_y_best,
    );
    ores.save("ores.png").unwrap();

    Ok(())
}
