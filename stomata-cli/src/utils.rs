//! Utility functions for data conversion and formatting
//!
//! Provides helper functions for converting between different units and
//! formats commonly used in system monitoring displays.

/// Converts bytes to megabytes.
///
/// Performs binary conversion (1024-based) from bytes to megabytes,
/// returning a floating-point value for precision in display.
///
/// # Arguments
///
/// * `bytes` - Number of bytes to convert
///
/// # Returns
///
/// The equivalent value in megabytes as an f64
///
/// # Conversion
///
/// Uses binary (base-1024) conversion:
/// - 1 KB = 1,024 bytes
/// - 1 MB = 1,024 KB = 1,048,576 bytes

pub fn bytes_to_mb(bytes: u64) -> f64 {
    (bytes as f64) / (1024.0 * 1024.0)
}
