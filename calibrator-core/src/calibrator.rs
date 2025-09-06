//! Top Level Camera Calibration

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
    
}