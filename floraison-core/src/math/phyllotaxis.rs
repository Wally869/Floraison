//! Phyllotaxis - botanical arrangement patterns
//!
//! This module implements various phyllotactic (leaf/component arrangement) patterns
//! found in nature, particularly Fibonacci spirals and related arrangements.
//!
//! # Phyllotaxis in Flowers
//!
//! Natural flowers arrange their components (pistils, stamens, petals, sepals) in
//! mathematically precise patterns. The most common is the Fibonacci spiral using
//! the golden angle (~137.5°), which maximizes packing efficiency and creates
//! aesthetically pleasing arrangements.
//!
//! # Examples
//!
//! ```
//! use floraison_core::math::phyllotaxis::{vogel_spiral, radial_positions, GOLDEN_ANGLE};
//!
//! // Arrange 21 stamens in a Fibonacci spiral
//! let positions: Vec<_> = (0..21)
//!     .map(|i| vogel_spiral(i, 21, 1.0))
//!     .collect();
//!
//! // Arrange 5 petals evenly in a circle
//! let petals = radial_positions(5, 2.0, 0.0);
//! ```

use crate::{Vec2, Vec3};
use std::f32::consts::PI;

/// Golden angle in radians (≈ 2.399963 rad ≈ 137.5078°)
///
/// The golden angle is the angle that divides a circle in the golden ratio.
/// It's commonly found in nature for optimal packing of seeds, petals, and leaves.
///
/// Formula: π(3 - √5) ≈ 2.39996322972865332 radians ≈ 137.5077640500378°
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::GOLDEN_ANGLE;
/// use std::f32::consts::PI;
///
/// // Verify it's approximately 137.5 degrees
/// let degrees = GOLDEN_ANGLE * 180.0 / PI;
/// assert!((degrees - 137.5078).abs() < 0.001);
/// ```
pub const GOLDEN_ANGLE: f32 = 2.39996322972865332;

/// Alternate phyllotaxis: 180° divergence (opposite arrangement)
///
/// Example: leaves on opposite sides of a stem
pub const ANGLE_180: f32 = PI;

/// Decussate phyllotaxis: 90° divergence (perpendicular pairs)
///
/// Example: Mentha (mint) leaf arrangement
pub const ANGLE_90: f32 = PI / 2.0;

/// Tricussate phyllotaxis: 120° divergence (three-way symmetry)
///
/// Example: some Aloe species
pub const ANGLE_120: f32 = 2.0 * PI / 3.0;

/// Pentagonal phyllotaxis: 144° divergence (5-fold symmetry)
///
/// Example: some flower petals
pub const ANGLE_144: f32 = 2.0 * PI * 2.0 / 5.0;

/// Calculate the Fibonacci spiral angle for a given index
///
/// This is simply the golden angle multiplied by the index, wrapping around 2π.
///
/// # Arguments
/// * `index` - Position in the spiral sequence (0, 1, 2, ...)
///
/// # Returns
/// Angle in radians, normalized to [0, 2π)
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::fibonacci_angle;
/// use std::f32::consts::PI;
///
/// let angle = fibonacci_angle(0);
/// assert_eq!(angle, 0.0);
///
/// let angle = fibonacci_angle(1);
/// assert!(angle > 0.0 && angle < 2.0 * PI);
/// ```
pub fn fibonacci_angle(index: usize) -> f32 {
    let angle = (index as f32 * GOLDEN_ANGLE) % (2.0 * PI);
    angle
}

/// Calculate 2D position using Vogel's method for optimal disc packing
///
/// Vogel's method creates a Fibonacci spiral that optimally packs circular elements
/// in a disc. The radius grows as √(i/n) to maintain uniform density.
///
/// This is ideal for arranging indefinite numbers of components like stamens in
/// flowers such as Ranunculus (buttercup).
///
/// # Arguments
/// * `index` - Element index in sequence (0 to count-1)
/// * `count` - Total number of elements
/// * `radius` - Maximum radius of the arrangement
///
/// # Returns
/// 2D position in the disc
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::vogel_spiral;
///
/// // Arrange 100 points in a disc of radius 5.0
/// let positions: Vec<_> = (0..100)
///     .map(|i| vogel_spiral(i, 100, 5.0))
///     .collect();
///
/// // First point is at center
/// assert!(positions[0].length() < 0.1);
///
/// // Last point is near the edge
/// assert!((positions[99].length() - 5.0).abs() < 0.5);
/// ```
pub fn vogel_spiral(index: usize, count: usize, radius: f32) -> Vec2 {
    let angle = index as f32 * GOLDEN_ANGLE;
    let r = if count > 1 {
        radius * (index as f32 / (count - 1) as f32).sqrt()
    } else {
        0.0
    };
    Vec2::new(r * angle.cos(), r * angle.sin())
}

/// Calculate positions for radial arrangement (evenly spaced around a circle)
///
/// Places elements at equal angular intervals around a circle, optionally
/// rotated by an offset angle. This is used for regular flower arrangements
/// like 5-petaled flowers.
///
/// # Arguments
/// * `count` - Number of elements to arrange
/// * `radius` - Distance from center
/// * `angle_offset` - Initial rotation offset in radians (default: 0.0 for first element at 0°)
///
/// # Returns
/// Vector of 2D positions
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::radial_positions;
///
/// // 5 petals in a regular pentagon
/// let petals = radial_positions(5, 2.0, 0.0);
/// assert_eq!(petals.len(), 5);
///
/// // First petal at 0 degrees (positive X)
/// assert!((petals[0].x - 2.0).abs() < 0.001);
/// assert!(petals[0].y.abs() < 0.001);
/// ```
pub fn radial_positions(count: usize, radius: f32, angle_offset: f32) -> Vec<Vec2> {
    if count == 0 {
        return Vec::new();
    }

    let angle_step = 2.0 * PI / count as f32;
    (0..count)
        .map(|i| {
            let angle = i as f32 * angle_step + angle_offset;
            Vec2::new(radius * angle.cos(), radius * angle.sin())
        })
        .collect()
}

/// Calculate positions for whorled arrangement (multiple elements at same height)
///
/// A whorl is a ring of elements at the same height/radius. This is used for
/// flowers with petals or stamens in distinct rings.
///
/// # Arguments
/// * `count` - Number of elements in the whorl
/// * `radius` - Distance from center axis
/// * `height` - Height along the vertical axis
/// * `angle_offset` - Initial rotation offset in radians
///
/// # Returns
/// Vector of 3D positions forming a horizontal ring
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::whorled_positions;
///
/// // 6 stamens in a whorl at height 1.0, radius 0.5
/// let stamens = whorled_positions(6, 0.5, 1.0, 0.0);
/// assert_eq!(stamens.len(), 6);
///
/// // All at same height
/// for pos in &stamens {
///     assert!((pos.y - 1.0).abs() < 0.001);
/// }
/// ```
pub fn whorled_positions(count: usize, radius: f32, height: f32, angle_offset: f32) -> Vec<Vec3> {
    if count == 0 {
        return Vec::new();
    }

    let angle_step = 2.0 * PI / count as f32;
    (0..count)
        .map(|i| {
            let angle = i as f32 * angle_step + angle_offset;
            Vec3::new(radius * angle.cos(), height, radius * angle.sin())
        })
        .collect()
}

/// Calculate 3D positions using Fibonacci spiral on a cylinder
///
/// Creates a spiral arrangement along a cylinder, useful for leaf arrangement
/// on stems or spiral inflorescences.
///
/// # Arguments
/// * `count` - Number of elements
/// * `base_radius` - Cylinder radius
/// * `height` - Total height of the arrangement
/// * `radius_fn` - Optional function to vary radius with normalized position [0,1]
///
/// # Returns
/// Vector of 3D positions
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::fibonacci_spiral_3d;
///
/// // 10 leaves spiraling up a stem
/// let leaves = fibonacci_spiral_3d(10, 0.5, 5.0, None);
/// assert_eq!(leaves.len(), 10);
///
/// // First leaf at bottom
/// assert!(leaves[0].y < 0.1);
///
/// // Last leaf near top
/// assert!((leaves[9].y - 5.0).abs() < 0.6);
/// ```
pub fn fibonacci_spiral_3d(
    count: usize,
    base_radius: f32,
    height: f32,
    radius_fn: Option<fn(f32) -> f32>,
) -> Vec<Vec3> {
    if count == 0 {
        return Vec::new();
    }

    (0..count)
        .map(|i| {
            let t = if count > 1 {
                i as f32 / (count - 1) as f32
            } else {
                0.0
            };
            let angle = fibonacci_angle(i);
            let y = t * height;

            let radius = if let Some(f) = radius_fn {
                base_radius * f(t)
            } else {
                base_radius
            };

            Vec3::new(radius * angle.cos(), y, radius * angle.sin())
        })
        .collect()
}

/// Linear radius variation function (constant radius)
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::radius_constant;
///
/// assert_eq!(radius_constant(0.0), 1.0);
/// assert_eq!(radius_constant(0.5), 1.0);
/// assert_eq!(radius_constant(1.0), 1.0);
/// ```
pub fn radius_constant(_t: f32) -> f32 {
    1.0
}

/// Linear radius variation (cone shape)
///
/// Radius decreases linearly from 1.0 at base to 0.0 at top.
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::radius_linear;
///
/// assert_eq!(radius_linear(0.0), 1.0);
/// assert_eq!(radius_linear(0.5), 0.5);
/// assert_eq!(radius_linear(1.0), 0.0);
/// ```
pub fn radius_linear(t: f32) -> f32 {
    1.0 - t
}

/// Quadratic radius variation (smooth cone)
///
/// Radius decreases quadratically for a smooth taper.
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::radius_quadratic;
///
/// assert_eq!(radius_quadratic(0.0), 1.0);
/// assert!((radius_quadratic(0.5) - 0.25).abs() < 0.001);
/// assert_eq!(radius_quadratic(1.0), 0.0);
/// ```
pub fn radius_quadratic(t: f32) -> f32 {
    let s = 1.0 - t;
    s * s
}

/// Bulge radius variation (wider in middle)
///
/// Creates a bulge in the middle of the arrangement.
///
/// # Example
/// ```
/// use floraison_core::math::phyllotaxis::radius_bulge;
///
/// assert!(radius_bulge(0.0).abs() < 0.001); // Near 0 at start
/// assert!(radius_bulge(0.5) > 0.9); // Near 1.0 in middle
/// assert!(radius_bulge(1.0).abs() < 0.001); // Near 0 at end
/// ```
pub fn radius_bulge(t: f32) -> f32 {
    // Smooth bell curve using sin
    (t * PI).sin()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    #[test]
    fn test_golden_angle_value() {
        // Golden angle should be approximately 137.5078 degrees
        let degrees = GOLDEN_ANGLE * 180.0 / PI;
        assert!((degrees - 137.5078).abs() < 0.001);

        // Should be approximately 2.399963 radians
        assert!((GOLDEN_ANGLE - 2.399963).abs() < 0.0001);
    }

    #[test]
    fn test_fibonacci_angle() {
        let angle0 = fibonacci_angle(0);
        assert_eq!(angle0, 0.0);

        let angle1 = fibonacci_angle(1);
        assert!((angle1 - GOLDEN_ANGLE).abs() < EPSILON);

        // Should wrap around 2π
        for i in 0..100 {
            let angle = fibonacci_angle(i);
            assert!(angle >= 0.0 && angle < 2.0 * PI);
        }
    }

    #[test]
    fn test_vogel_spiral_center() {
        let pos = vogel_spiral(0, 100, 5.0);
        // First point should be at or very near center
        assert!(pos.length() < EPSILON);
    }

    #[test]
    fn test_vogel_spiral_edge() {
        let pos = vogel_spiral(99, 100, 5.0);
        // Last point should be near the edge radius
        assert!((pos.length() - 5.0).abs() < 0.5);
    }

    #[test]
    fn test_vogel_spiral_single_element() {
        let pos = vogel_spiral(0, 1, 5.0);
        assert_eq!(pos, Vec2::ZERO);
    }

    #[test]
    fn test_vogel_spiral_uniform_density() {
        // Points should be roughly evenly distributed
        let positions: Vec<_> = (0..100).map(|i| vogel_spiral(i, 100, 10.0)).collect();

        // Check that points don't cluster too much
        // Average distance between consecutive points should be relatively uniform
        let mut distances = Vec::new();
        for i in 1..positions.len() {
            let dist = (positions[i] - positions[i - 1]).length();
            distances.push(dist);
        }

        let avg_dist: f32 = distances.iter().sum::<f32>() / distances.len() as f32;
        // Most distances should be within 2x of average (rough check)
        let within_tolerance = distances
            .iter()
            .filter(|&&d| d < avg_dist * 2.0 && d > avg_dist * 0.5)
            .count();
        assert!(within_tolerance as f32 / distances.len() as f32 > 0.7);
    }

    #[test]
    fn test_radial_positions_count() {
        let positions = radial_positions(5, 2.0, 0.0);
        assert_eq!(positions.len(), 5);
    }

    #[test]
    fn test_radial_positions_zero() {
        let positions = radial_positions(0, 2.0, 0.0);
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn test_radial_positions_radius() {
        let positions = radial_positions(8, 3.0, 0.0);
        for pos in &positions {
            // All points should be at radius 3.0
            assert!((pos.length() - 3.0).abs() < EPSILON);
        }
    }

    #[test]
    fn test_radial_positions_first_element() {
        let positions = radial_positions(4, 1.0, 0.0);
        // First element should be at angle 0 (positive X)
        assert!((positions[0].x - 1.0).abs() < EPSILON);
        assert!(positions[0].y.abs() < EPSILON);
    }

    #[test]
    fn test_radial_positions_offset() {
        let positions = radial_positions(4, 1.0, PI / 4.0);
        // First element should be at 45 degrees
        assert!((positions[0].x - 0.707107).abs() < 0.001);
        assert!((positions[0].y - 0.707107).abs() < 0.001);
    }

    #[test]
    fn test_whorled_positions_count() {
        let positions = whorled_positions(6, 0.5, 1.0, 0.0);
        assert_eq!(positions.len(), 6);
    }

    #[test]
    fn test_whorled_positions_height() {
        let positions = whorled_positions(8, 0.5, 2.5, 0.0);
        for pos in &positions {
            // All points should be at same height
            assert!((pos.y - 2.5).abs() < EPSILON);
        }
    }

    #[test]
    fn test_whorled_positions_radius() {
        let positions = whorled_positions(10, 1.5, 0.0, 0.0);
        for pos in &positions {
            // All points should be at radius 1.5 from Y axis
            let r = (pos.x * pos.x + pos.z * pos.z).sqrt();
            assert!((r - 1.5).abs() < EPSILON);
        }
    }

    #[test]
    fn test_fibonacci_spiral_3d_count() {
        let positions = fibonacci_spiral_3d(10, 0.5, 5.0, None);
        assert_eq!(positions.len(), 10);
    }

    #[test]
    fn test_fibonacci_spiral_3d_height_range() {
        let positions = fibonacci_spiral_3d(20, 0.5, 10.0, None);

        // First element should be at bottom
        assert!(positions[0].y < 0.1);

        // Last element should be at top
        assert!((positions[19].y - 10.0).abs() < 0.6);

        // Heights should be monotonically increasing
        for i in 1..positions.len() {
            assert!(positions[i].y >= positions[i - 1].y);
        }
    }

    #[test]
    fn test_fibonacci_spiral_3d_radius_constant() {
        let positions = fibonacci_spiral_3d(15, 1.0, 5.0, Some(radius_constant));
        for pos in &positions {
            let r = (pos.x * pos.x + pos.z * pos.z).sqrt();
            assert!((r - 1.0).abs() < EPSILON);
        }
    }

    #[test]
    fn test_fibonacci_spiral_3d_radius_linear() {
        let positions = fibonacci_spiral_3d(10, 2.0, 5.0, Some(radius_linear));

        // First element (t=0) should have full radius
        let r0 = (positions[0].x * positions[0].x + positions[0].z * positions[0].z).sqrt();
        assert!((r0 - 2.0).abs() < EPSILON);

        // Last element (t=1) should have zero radius
        let r_last = (positions[9].x * positions[9].x + positions[9].z * positions[9].z).sqrt();
        assert!(r_last < EPSILON);
    }

    #[test]
    fn test_radius_functions() {
        assert_eq!(radius_constant(0.5), 1.0);
        assert_eq!(radius_linear(0.5), 0.5);
        assert!((radius_quadratic(0.5) - 0.25).abs() < EPSILON);

        // Bulge should peak in middle
        assert!(radius_bulge(0.5) > radius_bulge(0.0));
        assert!(radius_bulge(0.5) > radius_bulge(1.0));
    }

    #[test]
    fn test_common_angles() {
        assert_eq!(ANGLE_180, PI);
        assert_eq!(ANGLE_90, PI / 2.0);
        assert!((ANGLE_120 - 2.0944).abs() < 0.001); // 120° in radians
        assert!((ANGLE_144 - 2.5133).abs() < 0.001); // 144° in radians
    }

    #[test]
    fn test_single_element_arrangements() {
        let radial = radial_positions(1, 1.0, 0.0);
        assert_eq!(radial.len(), 1);

        let whorled = whorled_positions(1, 1.0, 0.0, 0.0);
        assert_eq!(whorled.len(), 1);

        let spiral = fibonacci_spiral_3d(1, 1.0, 1.0, None);
        assert_eq!(spiral.len(), 1);
    }
}
