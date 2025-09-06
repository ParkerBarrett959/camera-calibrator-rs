//! Camera Calibrator Types

/// Point 2D
///
/// This struct contains a unitless representation of a 2D point.
///
/// # Fields
///
/// * `x` - The x coordinate of the point
/// * `y` - The y coordinate of the point
#[derive(Debug, Clone, Copy, Default)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

/// Correspondence
///
/// This struct contains a correspondence pair, which is one world frame point with
/// one image point. Since this crate uses Zhang's method, it is assumed that a 2D
/// planar image is being used for calibration, therefore the third dimension of the
/// world coordinates, Z = 0, and is dropped for brevity.
///
/// # Fields
///
/// * `world` - The world frame point
/// * `image` - The image frame point
#[derive(Debug, Clone, Copy, Default)]
pub struct Correspondence {
    pub world: Point2,
    pub image: Point2,
}

/// Image Data
///
/// This struct contains all the correspondence pairs from a single image.
///
/// # Fields
///
/// * `points` - The vector of correspondences from a single image
#[derive(Debug, Clone, Default)]
pub struct ImageData {
    pub points: Vec<Correspondence>,
}

/// Calibration Data
///
/// This struct contains a vector of each image used in the calibration.
///
/// # Fields
///
/// * `images` - The vector of ImageData
#[derive(Debug, Clone, Default)]
pub struct CalibrationData {
    pub images: Vec<ImageData>,
}
