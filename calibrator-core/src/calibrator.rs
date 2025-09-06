//! Top Level Camera Calibration
use crate::types::CalibrationData;

/// Calibrator struct definition
///
/// The calibrator struct is the top level component for calibrating a camera based
/// on a series of images using Zhang's method. At least two images are required from
/// different perspectives to fully constrain the problem (assuming no image skew),
/// however more are recommended. It is assumed that image processing has already been
/// applied to the collected images, and data can be passed to the algorithm as pairs
/// of world coordinates with corresponding image coordinates.
///
/// # Fields
///
/// * `tmp` - TODO
/// * `tmp2` - TODO
#[derive(Debug, Clone, Copy, Default)]
pub struct Calibrator {
    // TODO
}

impl Calibrator {
    /// Solve calibration
    ///
    /// # Arguments
    ///
    /// * `cal_data` - The calibration data to use in the solve
    ///
    /// # Returns
    ///
    /// Returns a calibration result which contains camera intrinsics/distortion coefficients
    pub fn solve(cal_data: &CalibrationData) -> f64 {
        let tmp: f64 = 1.0;
        tmp
    }
}
