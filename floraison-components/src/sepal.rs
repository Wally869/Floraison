//! Sepal (protective leaf-like structure) generator
//!
//! Sepals are the outermost floral components that protect the flower bud before it opens.
//! Structurally, sepals are very similar to petals, so this module reuses the petal
//! generator with sepal-appropriate default parameters.
//!
//! # Relationship to Petals
//!
//! Botanically, sepals and petals are both modified leaves and share similar geometry.
//! The main differences are:
//! - Sepals are typically green (chlorophyll-containing)
//! - Sepals are narrower and more protective
//! - Sepals often curl downward or outward
//! - Sepals appear in the outermost whorl
//!
//! # Example
//! ```
//! use floraison_components::sepal::{PetalParams, generate};
//!
//! // Use a sepal preset
//! let params = floraison_components::sepal::default();
//!
//! let sepal = generate(&params);
//! assert!(sepal.vertex_count() > 0);
//! ```

// Re-export petal types and functions
pub use crate::petal::{PetalParams, generate};

/// Create default parameters for a typical sepal
///
/// Sepals are narrower than petals with a slight downward curl
/// for a protective appearance.
///
/// # Returns
/// PetalParams configured for a typical sepal
///
/// # Example
/// ```
/// use floraison_components::sepal;
///
/// let params = sepal::default();
/// let sepal = sepal::generate(&params);
/// ```
pub fn default() -> PetalParams {
    PetalParams {
        length: 3.0,
        width: 1.0,
        tip_sharpness: 0.5,
        base_width: 0.4,
        curl: -0.2,        // Slight downward curl
        twist: 0.0,
        ruffle_freq: 0.0,
        ruffle_amp: 0.0,
        lateral_curve: 0.0,
        resolution: 16,
        color: crate::Vec3::new(0.2, 0.6, 0.2),  // Green
    }
}

/// Create parameters for a narrow, protective sepal
///
/// These sepals are thin and tightly curved for maximum protection.
///
/// # Returns
/// PetalParams configured for narrow sepals
///
/// # Example
/// ```
/// use floraison_components::sepal;
///
/// let params = sepal::narrow();
/// let sepal = sepal::generate(&params);
/// ```
pub fn narrow() -> PetalParams {
    PetalParams {
        length: 3.5,
        width: 0.7,
        tip_sharpness: 0.7,
        base_width: 0.3,
        curl: -0.3,        // More pronounced downward curl
        twist: 0.0,
        ruffle_freq: 0.0,
        ruffle_amp: 0.0,
        lateral_curve: 0.0,
        resolution: 14,
        color: crate::Vec3::new(0.2, 0.6, 0.2),  // Green
    }
}

/// Create parameters for a wide, leafy sepal
///
/// These sepals are broader and flatter, resembling leaves.
///
/// # Returns
/// PetalParams configured for wide sepals
///
/// # Example
/// ```
/// use floraison_components::sepal;
///
/// let params = sepal::wide();
/// let sepal = sepal::generate(&params);
/// ```
pub fn wide() -> PetalParams {
    PetalParams {
        length: 2.5,
        width: 1.5,
        tip_sharpness: 0.3,
        base_width: 0.6,
        curl: -0.1,        // Gentle outward curl
        twist: 0.0,
        ruffle_freq: 0.0,
        ruffle_amp: 0.0,
        lateral_curve: 0.0,
        resolution: 18,
        color: crate::Vec3::new(0.2, 0.6, 0.2),  // Green
    }
}

/// Create parameters for a recurved sepal
///
/// These sepals curl strongly backward away from the flower.
///
/// # Returns
/// PetalParams configured for recurved sepals
///
/// # Example
/// ```
/// use floraison_components::sepal;
///
/// let params = sepal::recurved();
/// let sepal = sepal::generate(&params);
/// ```
pub fn recurved() -> PetalParams {
    PetalParams {
        length: 3.0,
        width: 1.2,
        tip_sharpness: 0.4,
        base_width: 0.5,
        curl: -0.6,        // Strong backward curl
        twist: 5.0,        // Slight twist for organic look
        ruffle_freq: 0.0,
        ruffle_amp: 0.0,
        lateral_curve: 0.0,
        resolution: 16,
        color: crate::Vec3::new(0.2, 0.6, 0.2),  // Green
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_sepal() {
        let params = default();
        let mesh = generate(&params);

        assert!(mesh.vertex_count() > 0, "Should have vertices");
        assert!(mesh.triangle_count() > 0, "Should have triangles");

        // Verify default has downward curl
        assert!(params.curl < 0.0, "Default sepal should curl downward");
    }

    #[test]
    fn test_narrow_sepal() {
        let params = narrow();
        let mesh = generate(&params);

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);

        // Narrow sepal should be thinner than default
        let default_params = default();
        assert!(params.width < default_params.width, "Narrow sepal should be narrower");
        assert!(params.curl < default_params.curl, "Narrow sepal should curl more");
    }

    #[test]
    fn test_wide_sepal() {
        let params = wide();
        let mesh = generate(&params);

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);

        // Wide sepal should be broader than default
        let default_params = default();
        assert!(params.width > default_params.width, "Wide sepal should be wider");
    }

    #[test]
    fn test_recurved_sepal() {
        let params = recurved();
        let mesh = generate(&params);

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);

        // Recurved sepal should have strong curl
        assert!(params.curl < -0.5, "Recurved sepal should curl strongly backward");
    }

    #[test]
    fn test_all_sepals_curl_downward() {
        // All sepal presets should have negative (downward/outward) curl
        assert!(default().curl < 0.0, "Default sepal curls downward");
        assert!(narrow().curl < 0.0, "Narrow sepal curls downward");
        assert!(wide().curl < 0.0, "Wide sepal curls downward");
        assert!(recurved().curl < 0.0, "Recurved sepal curls downward");
    }

    #[test]
    fn test_sepal_geometry_valid() {
        let params = default();
        let mesh = generate(&params);

        // Check geometry validity
        for pos in &mesh.positions {
            assert!(pos.is_finite(), "Position should be finite");
        }

        for normal in &mesh.normals {
            assert!(normal.is_finite(), "Normal should be finite");
            let len = normal.length();
            assert!(
                (len - 1.0).abs() < 0.1,
                "Normal should be normalized, got length {}",
                len
            );
        }
    }
}
