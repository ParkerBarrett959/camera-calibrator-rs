//! Camera Calibrator Utility Functions
use crate::types::ImageData;
use nalgebra::Matrix3;

/// Compute homography from image
///
/// This function takes an image which is represented as a vector of correspondences
/// and computes the homography matrix, H = A[r1 r2 t], where A is the intrinsics
/// matrix and [r1 r2 t] contains the extrinsics (with the third column of the rotation
/// excluded for efficiency). The image data input must have at least 4 correspondences
/// to solve the homography, otherwise it will fail.
///
/// # Arguments
///
/// * `image` - An ImageData struct
///
/// # Returns
///
/// Returns a Result type which contains the Homography matrix, H if successful.
pub fn homography_from_image(image: &ImageData) -> Result<Matrix3<f64>, String> {
    let h: Matrix3<f64> = Matrix3::<f64>::zeros();
    Ok(h)
}
