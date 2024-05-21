use opencv::{
    core::{Mat, MatTraitConst, Size},
    imgproc,
    prelude::*,
    Result,
};

fn main() -> Result<()> {
    // Load the images
    let img1 = imgproc::imread("./image_stitching/1.png", imgproc::IMREAD_GRAYSCALE)?;
    let img2 = imgproc::imread("./image_stitching/2.png", imgproc::IMREAD_GRAYSCALE)?;
    // Convert images to floating point
    let mut img1_f32 = Mat::default()?;
    img1.convert_to(&mut img1_f32, opencv::core::CV_32F, 1.0, 0.0)?;

    let mut img2_f32 = Mat::default()?;
    img2.convert_to(&mut img2_f32, opencv::core::CV_32F, 1.0, 0.0)?;

    // Perform cross-correlation
    let mut result = Mat::default()?;
    imgproc::match_template(
        &img1_f32,
        &img2_f32,
        &mut result,
        imgproc::TM_CCORR_NORMED,
        &opencv::core::no_array()?,
    )?;

    // Find the best match position
    let mut min_val = 0.0;
    let mut max_val = 0.0;
    let mut min_loc = opencv::core::Point::default();
    let mut max_loc = opencv::core::Point::default();
    opencv::core::min_max_loc(
        &result,
        Some(&mut min_val),
        Some(&mut max_val),
        Some(&mut min_loc),
        Some(&mut max_loc),
        &opencv::core::no_array()?,
    )?;

    println!("Best match location: {:?}", max_loc);
    

    Ok(())
}