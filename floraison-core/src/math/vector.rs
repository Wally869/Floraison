//! Vector math extensions and coordinate system utilities
//!
//! This module provides helper functions for common operations in flower generation,
//! including coordinate system conversions and vector transformations.

use glam::{Vec2, Vec3};

/// Extensions for Vec3 to support cylindrical and spherical coordinates
pub trait Vec3Ext {
    /// Create a Vec3 from cylindrical coordinates (radius, angle, height)
    ///
    /// # Arguments
    /// * `radius` - Distance from the z-axis
    /// * `angle` - Angle in radians (counter-clockwise from x-axis when viewed from above)
    /// * `height` - Height along the z-axis
    ///
    /// # Example
    /// ```
    /// use floraison_core::math::vector::Vec3Ext;
    /// use glam::Vec3;
    /// use std::f32::consts::PI;
    ///
    /// let pos = Vec3::from_cylindrical(1.0, PI / 2.0, 0.5);
    /// assert!((pos.x - 0.0).abs() < 0.001);
    /// assert!((pos.y - 1.0).abs() < 0.001);
    /// assert!((pos.z - 0.5).abs() < 0.001);
    /// ```
    fn from_cylindrical(radius: f32, angle: f32, height: f32) -> Self;

    /// Create a Vec3 from spherical coordinates (radius, theta, phi)
    ///
    /// # Arguments
    /// * `radius` - Distance from origin
    /// * `theta` - Azimuthal angle in radians (counter-clockwise from x-axis in xy-plane)
    /// * `phi` - Polar angle in radians (angle from positive z-axis)
    ///
    /// # Example
    /// ```
    /// use floraison_core::math::vector::Vec3Ext;
    /// use glam::Vec3;
    /// use std::f32::consts::PI;
    ///
    /// let pos = Vec3::from_spherical(1.0, 0.0, PI / 2.0);
    /// assert!((pos.x - 1.0).abs() < 0.001);
    /// assert!((pos.y - 0.0).abs() < 0.001);
    /// assert!((pos.z - 0.0).abs() < 0.001);
    /// ```
    fn from_spherical(radius: f32, theta: f32, phi: f32) -> Self;

    /// Convert Vec3 to cylindrical coordinates (radius, angle, height)
    ///
    /// Returns (radius, angle, height) where:
    /// - radius is distance from z-axis
    /// - angle is in radians, range [0, 2π)
    /// - height is the z component
    ///
    /// # Example
    /// ```
    /// use floraison_core::math::vector::Vec3Ext;
    /// use glam::Vec3;
    /// use std::f32::consts::PI;
    ///
    /// let pos = Vec3::new(0.0, 1.0, 0.5);
    /// let (r, angle, h) = pos.to_cylindrical();
    /// assert!((r - 1.0).abs() < 0.001);
    /// assert!((angle - PI / 2.0).abs() < 0.001);
    /// assert!((h - 0.5).abs() < 0.001);
    /// ```
    fn to_cylindrical(&self) -> (f32, f32, f32);

    /// Convert Vec3 to spherical coordinates (radius, theta, phi)
    ///
    /// Returns (radius, theta, phi) where:
    /// - radius is distance from origin
    /// - theta is azimuthal angle in [0, 2π)
    /// - phi is polar angle in [0, π]
    ///
    /// # Example
    /// ```
    /// use floraison_core::math::vector::Vec3Ext;
    /// use glam::Vec3;
    /// use std::f32::consts::PI;
    ///
    /// let pos = Vec3::new(1.0, 0.0, 0.0);
    /// let (r, theta, phi) = pos.to_spherical();
    /// assert!((r - 1.0).abs() < 0.001);
    /// assert!((theta - 0.0).abs() < 0.001);
    /// assert!((phi - PI / 2.0).abs() < 0.001);
    /// ```
    fn to_spherical(&self) -> (f32, f32, f32);
}

impl Vec3Ext for Vec3 {
    fn from_cylindrical(radius: f32, angle: f32, height: f32) -> Self {
        Vec3::new(radius * angle.cos(), radius * angle.sin(), height)
    }

    fn from_spherical(radius: f32, theta: f32, phi: f32) -> Self {
        let sin_phi = phi.sin();
        Vec3::new(
            radius * sin_phi * theta.cos(),
            radius * sin_phi * theta.sin(),
            radius * phi.cos(),
        )
    }

    fn to_cylindrical(&self) -> (f32, f32, f32) {
        let radius = (self.x * self.x + self.y * self.y).sqrt();
        let angle = self.y.atan2(self.x);
        let angle = if angle < 0.0 {
            angle + 2.0 * std::f32::consts::PI
        } else {
            angle
        };
        (radius, angle, self.z)
    }

    fn to_spherical(&self) -> (f32, f32, f32) {
        let radius = self.length();
        let theta = self.y.atan2(self.x);
        let theta = if theta < 0.0 {
            theta + 2.0 * std::f32::consts::PI
        } else {
            theta
        };
        let phi = if radius > 0.0 {
            (self.z / radius).acos()
        } else {
            0.0
        };
        (radius, theta, phi)
    }
}

/// Extensions for Vec2 for 2D profile operations
pub trait Vec2Ext {
    /// Rotate a 2D vector by an angle in radians
    ///
    /// # Arguments
    /// * `angle` - Rotation angle in radians (counter-clockwise)
    ///
    /// # Example
    /// ```
    /// use floraison_core::math::vector::Vec2Ext;
    /// use glam::Vec2;
    /// use std::f32::consts::PI;
    ///
    /// let v = Vec2::new(1.0, 0.0);
    /// let rotated = v.rotate_by_angle(PI / 2.0);
    /// assert!((rotated.x - 0.0).abs() < 0.001);
    /// assert!((rotated.y - 1.0).abs() < 0.001);
    /// ```
    fn rotate_by_angle(&self, angle: f32) -> Self;

    /// Create a Vec2 from polar coordinates (radius, angle)
    ///
    /// # Arguments
    /// * `radius` - Distance from origin
    /// * `angle` - Angle in radians (counter-clockwise from x-axis)
    ///
    /// # Example
    /// ```
    /// use floraison_core::math::vector::Vec2Ext;
    /// use glam::Vec2;
    /// use std::f32::consts::PI;
    ///
    /// let pos = Vec2::from_polar(1.0, PI / 2.0);
    /// assert!((pos.x - 0.0).abs() < 0.001);
    /// assert!((pos.y - 1.0).abs() < 0.001);
    /// ```
    fn from_polar(radius: f32, angle: f32) -> Self;

    /// Convert Vec2 to polar coordinates (radius, angle)
    ///
    /// Returns (radius, angle) where angle is in [0, 2π)
    ///
    /// # Example
    /// ```
    /// use floraison_core::math::vector::Vec2Ext;
    /// use glam::Vec2;
    /// use std::f32::consts::PI;
    ///
    /// let pos = Vec2::new(0.0, 1.0);
    /// let (r, angle) = pos.to_polar();
    /// assert!((r - 1.0).abs() < 0.001);
    /// assert!((angle - PI / 2.0).abs() < 0.001);
    /// ```
    fn to_polar(&self) -> (f32, f32);
}

impl Vec2Ext for Vec2 {
    fn rotate_by_angle(&self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec2::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    fn from_polar(radius: f32, angle: f32) -> Self {
        Vec2::new(radius * angle.cos(), radius * angle.sin())
    }

    fn to_polar(&self) -> (f32, f32) {
        let radius = self.length();
        let angle = self.y.atan2(self.x);
        let angle = if angle < 0.0 {
            angle + 2.0 * std::f32::consts::PI
        } else {
            angle
        };
        (radius, angle)
    }
}

/// Linear interpolation between two values
///
/// # Arguments
/// * `a` - Start value
/// * `b` - End value
/// * `t` - Interpolation parameter in [0, 1]
///
/// # Example
/// ```
/// use floraison_core::math::vector::lerp;
///
/// assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
/// assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
/// assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
/// ```
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Smooth hermite interpolation (smoothstep)
///
/// Returns 0 for t <= 0, 1 for t >= 1, and smooth interpolation in between
///
/// # Arguments
/// * `t` - Input value
///
/// # Example
/// ```
/// use floraison_core::math::vector::smoothstep;
///
/// assert_eq!(smoothstep(0.0), 0.0);
/// assert_eq!(smoothstep(1.0), 1.0);
/// assert!((smoothstep(0.5) - 0.5).abs() < 0.001);
/// ```
#[inline]
pub fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Remap a value from one range to another
///
/// # Arguments
/// * `value` - Input value in range [in_min, in_max]
/// * `in_min` - Input range minimum
/// * `in_max` - Input range maximum
/// * `out_min` - Output range minimum
/// * `out_max` - Output range maximum
///
/// # Example
/// ```
/// use floraison_core::math::vector::remap;
///
/// assert_eq!(remap(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
/// assert_eq!(remap(0.0, 0.0, 10.0, 0.0, 100.0), 0.0);
/// assert_eq!(remap(10.0, 0.0, 10.0, 0.0, 100.0), 100.0);
/// ```
#[inline]
pub fn remap(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let t = (value - in_min) / (in_max - in_min);
    lerp(out_min, out_max, t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    const EPSILON: f32 = 1e-5;

    #[test]
    fn test_vec3_cylindrical_roundtrip() {
        let original = Vec3::new(3.0, 4.0, 5.0);
        let (r, angle, h) = original.to_cylindrical();
        let reconstructed = Vec3::from_cylindrical(r, angle, h);

        assert!((original.x - reconstructed.x).abs() < EPSILON);
        assert!((original.y - reconstructed.y).abs() < EPSILON);
        assert!((original.z - reconstructed.z).abs() < EPSILON);
    }

    #[test]
    fn test_vec3_cylindrical_specific() {
        // Test cardinal directions
        let pos = Vec3::from_cylindrical(1.0, 0.0, 0.0);
        assert!((pos.x - 1.0).abs() < EPSILON);
        assert!((pos.y - 0.0).abs() < EPSILON);

        let pos = Vec3::from_cylindrical(1.0, PI / 2.0, 2.0);
        assert!((pos.x - 0.0).abs() < EPSILON);
        assert!((pos.y - 1.0).abs() < EPSILON);
        assert!((pos.z - 2.0).abs() < EPSILON);

        let pos = Vec3::from_cylindrical(2.0, PI, 1.0);
        assert!((pos.x - (-2.0)).abs() < EPSILON);
        assert!((pos.y - 0.0).abs() < EPSILON);
        assert!((pos.z - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_vec3_spherical_roundtrip() {
        let original = Vec3::new(1.0, 2.0, 3.0);
        let (r, theta, phi) = original.to_spherical();
        let reconstructed = Vec3::from_spherical(r, theta, phi);

        assert!((original.x - reconstructed.x).abs() < EPSILON);
        assert!((original.y - reconstructed.y).abs() < EPSILON);
        assert!((original.z - reconstructed.z).abs() < EPSILON);
    }

    #[test]
    fn test_vec3_spherical_specific() {
        // Test unit sphere points
        let pos = Vec3::from_spherical(1.0, 0.0, PI / 2.0); // X-axis
        assert!((pos.x - 1.0).abs() < EPSILON);
        assert!((pos.y - 0.0).abs() < EPSILON);
        assert!((pos.z - 0.0).abs() < EPSILON);

        let pos = Vec3::from_spherical(1.0, PI / 2.0, PI / 2.0); // Y-axis
        assert!((pos.x - 0.0).abs() < EPSILON);
        assert!((pos.y - 1.0).abs() < EPSILON);
        assert!((pos.z - 0.0).abs() < EPSILON);

        let pos = Vec3::from_spherical(1.0, 0.0, 0.0); // Z-axis (north pole)
        assert!((pos.x - 0.0).abs() < EPSILON);
        assert!((pos.y - 0.0).abs() < EPSILON);
        assert!((pos.z - 1.0).abs() < EPSILON);

        let pos = Vec3::from_spherical(1.0, 0.0, PI); // -Z-axis (south pole)
        assert!((pos.x - 0.0).abs() < EPSILON);
        assert!((pos.y - 0.0).abs() < EPSILON);
        assert!((pos.z - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_vec2_polar_roundtrip() {
        let original = Vec2::new(3.0, 4.0);
        let (r, angle) = original.to_polar();
        let reconstructed = Vec2::from_polar(r, angle);

        assert!((original.x - reconstructed.x).abs() < EPSILON);
        assert!((original.y - reconstructed.y).abs() < EPSILON);
    }

    #[test]
    fn test_vec2_rotation() {
        let v = Vec2::new(1.0, 0.0);

        let rot_90 = v.rotate_by_angle(PI / 2.0);
        assert!((rot_90.x - 0.0).abs() < EPSILON);
        assert!((rot_90.y - 1.0).abs() < EPSILON);

        let rot_180 = v.rotate_by_angle(PI);
        assert!((rot_180.x - (-1.0)).abs() < EPSILON);
        assert!((rot_180.y - 0.0).abs() < EPSILON);

        let rot_360 = v.rotate_by_angle(2.0 * PI);
        assert!((rot_360.x - 1.0).abs() < EPSILON);
        assert!((rot_360.y - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(-10.0, 10.0, 0.5), 0.0);
    }

    #[test]
    fn test_smoothstep() {
        assert_eq!(smoothstep(0.0), 0.0);
        assert_eq!(smoothstep(1.0), 1.0);
        assert_eq!(smoothstep(-1.0), 0.0);
        assert_eq!(smoothstep(2.0), 1.0);

        // Smoothstep should be smoother than linear at 0.5
        let mid = smoothstep(0.5);
        assert!((mid - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_remap() {
        assert_eq!(remap(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(remap(0.0, 0.0, 10.0, 0.0, 100.0), 0.0);
        assert_eq!(remap(10.0, 0.0, 10.0, 0.0, 100.0), 100.0);
        assert_eq!(remap(5.0, 0.0, 10.0, 100.0, 0.0), 50.0); // Reverse range
    }

    #[test]
    fn test_cylindrical_angle_normalization() {
        // Test that angles are normalized to [0, 2π)
        let pos = Vec3::new(-1.0, 0.0, 0.0);
        let (_, angle, _) = pos.to_cylindrical();
        assert!((0.0..2.0 * PI).contains(&angle));
        assert!((angle - PI).abs() < EPSILON);
    }

    #[test]
    fn test_spherical_angle_normalization() {
        // Test that theta is normalized to [0, 2π)
        let pos = Vec3::new(-1.0, 0.0, 0.0);
        let (_, theta, _) = pos.to_spherical();
        assert!((0.0..2.0 * PI).contains(&theta));
        assert!((theta - PI).abs() < EPSILON);
    }

    #[test]
    fn test_zero_vector_spherical() {
        // Zero vector should return (0, 0, 0) without panicking
        let pos = Vec3::ZERO;
        let (r, _theta, phi) = pos.to_spherical();
        assert_eq!(r, 0.0);
        assert_eq!(phi, 0.0);
    }
}
