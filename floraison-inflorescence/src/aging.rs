//! Flower aging system
//!
//! Provides support for age-based flower appearance, allowing inflorescences
//! to display flowers at different developmental stages (bud, bloom, wilt).

use floraison_core::geometry::mesh::Mesh;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Flower aging configuration with meshes for different developmental stages
///
/// Allows inflorescences to show age progression from bud to bloom to wilt.
///
/// # Example
/// ```no_run
/// use floraison_inflorescence::aging::FlowerAging;
/// use floraison_core::geometry::mesh::Mesh;
///
/// let bud = Mesh::new();    // Closed bud mesh
/// let bloom = Mesh::new();  // Open flower mesh
/// let wilt = Mesh::new();   // Wilted flower mesh
///
/// let aging = FlowerAging {
///     bud_mesh: bud,
///     bloom_mesh: bloom,
///     wilt_mesh: Some(wilt),
/// };
///
/// // Select appropriate mesh based on age (0.0-1.0)
/// let young_flower = aging.select_mesh(0.2);  // Returns bud
/// let mature_flower = aging.select_mesh(0.6); // Returns bloom
/// let old_flower = aging.select_mesh(0.9);    // Returns wilt
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FlowerAging {
    /// Mesh for young/unopened flowers (bud stage)
    pub bud_mesh: Mesh,

    /// Mesh for mature/open flowers (full bloom)
    pub bloom_mesh: Mesh,

    /// Optional mesh for old/wilted flowers
    ///
    /// If `None`, `bloom_mesh` will be used for old flowers
    pub wilt_mesh: Option<Mesh>,
}

impl FlowerAging {
    /// Create a new aging configuration with bud and bloom meshes
    ///
    /// # Arguments
    /// * `bud_mesh` - Mesh representing young/unopened flower
    /// * `bloom_mesh` - Mesh representing mature/open flower
    ///
    /// # Example
    /// ```no_run
    /// use floraison_inflorescence::aging::FlowerAging;
    /// use floraison_core::geometry::mesh::Mesh;
    ///
    /// let bud = Mesh::new();
    /// let bloom = Mesh::new();
    ///
    /// let aging = FlowerAging::new(bud, bloom);
    /// ```
    pub fn new(bud_mesh: Mesh, bloom_mesh: Mesh) -> Self {
        Self {
            bud_mesh,
            bloom_mesh,
            wilt_mesh: None,
        }
    }

    /// Create aging configuration with all three stages (bud, bloom, wilt)
    ///
    /// # Arguments
    /// * `bud_mesh` - Mesh representing young/unopened flower
    /// * `bloom_mesh` - Mesh representing mature/open flower
    /// * `wilt_mesh` - Mesh representing old/wilted flower
    ///
    /// # Example
    /// ```no_run
    /// use floraison_inflorescence::aging::FlowerAging;
    /// use floraison_core::geometry::mesh::Mesh;
    ///
    /// let bud = Mesh::new();
    /// let bloom = Mesh::new();
    /// let wilt = Mesh::new();
    ///
    /// let aging = FlowerAging::with_wilt(bud, bloom, wilt);
    /// ```
    pub fn with_wilt(bud_mesh: Mesh, bloom_mesh: Mesh, wilt_mesh: Mesh) -> Self {
        Self {
            bud_mesh,
            bloom_mesh,
            wilt_mesh: Some(wilt_mesh),
        }
    }

    /// Select appropriate mesh based on flower age
    ///
    /// Uses discrete thresholds for stage transitions:
    /// - `age < 0.3`: bud stage
    /// - `0.3 <= age < 0.8`: bloom stage
    /// - `age >= 0.8`: wilt stage (if available)
    ///
    /// # Arguments
    /// * `age` - Normalized age value (0.0 = youngest, 1.0 = oldest)
    ///
    /// # Returns
    /// Reference to the appropriate mesh for this age
    ///
    /// # Example
    /// ```no_run
    /// use floraison_inflorescence::aging::FlowerAging;
    /// use floraison_core::geometry::mesh::Mesh;
    ///
    /// let aging = FlowerAging::new(Mesh::new(), Mesh::new());
    ///
    /// let mesh = aging.select_mesh(0.5); // Bloom stage
    /// ```
    pub fn select_mesh(&self, age: f32) -> &Mesh {
        if age < 0.3 {
            &self.bud_mesh
        } else if age < 0.8 {
            &self.bloom_mesh
        } else {
            // Use wilt if available, otherwise fallback to bloom
            self.wilt_mesh.as_ref().unwrap_or(&self.bloom_mesh)
        }
    }

    /// Get thresholds used for stage transitions
    ///
    /// # Returns
    /// Tuple of (bud_threshold, wilt_threshold)
    ///
    /// # Example
    /// ```
    /// use floraison_inflorescence::aging::FlowerAging;
    /// use floraison_core::geometry::mesh::Mesh;
    ///
    /// let aging = FlowerAging::new(Mesh::new(), Mesh::new());
    /// let (bud_threshold, wilt_threshold) = aging.thresholds();
    ///
    /// assert_eq!(bud_threshold, 0.3);
    /// assert_eq!(wilt_threshold, 0.8);
    /// ```
    pub fn thresholds(&self) -> (f32, f32) {
        (0.3, 0.8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use floraison_core::{Vec2, Vec3};

    fn create_test_mesh(vertex_count: usize) -> Mesh {
        let mut mesh = Mesh::new();
        for i in 0..vertex_count {
            mesh.add_vertex(
                Vec3::new(i as f32, 0.0, 0.0),
                Vec3::Y,
                Vec2::ZERO,
                Vec3::ONE,
            );
        }
        mesh
    }

    #[test]
    fn test_aging_new() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);

        let aging = FlowerAging::new(bud.clone(), bloom.clone());

        assert_eq!(aging.bud_mesh.vertex_count(), 3);
        assert_eq!(aging.bloom_mesh.vertex_count(), 5);
        assert!(aging.wilt_mesh.is_none());
    }

    #[test]
    fn test_aging_with_wilt() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);
        let wilt = create_test_mesh(4);

        let aging = FlowerAging::with_wilt(bud.clone(), bloom.clone(), wilt.clone());

        assert_eq!(aging.bud_mesh.vertex_count(), 3);
        assert_eq!(aging.bloom_mesh.vertex_count(), 5);
        assert!(aging.wilt_mesh.is_some());
        assert_eq!(aging.wilt_mesh.as_ref().unwrap().vertex_count(), 4);
    }

    #[test]
    fn test_select_mesh_bud_stage() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);
        let aging = FlowerAging::new(bud, bloom);

        // Young flowers should get bud mesh
        assert_eq!(aging.select_mesh(0.0).vertex_count(), 3);
        assert_eq!(aging.select_mesh(0.1).vertex_count(), 3);
        assert_eq!(aging.select_mesh(0.29).vertex_count(), 3);
    }

    #[test]
    fn test_select_mesh_bloom_stage() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);
        let aging = FlowerAging::new(bud, bloom);

        // Mature flowers should get bloom mesh
        assert_eq!(aging.select_mesh(0.3).vertex_count(), 5);
        assert_eq!(aging.select_mesh(0.5).vertex_count(), 5);
        assert_eq!(aging.select_mesh(0.79).vertex_count(), 5);
    }

    #[test]
    fn test_select_mesh_wilt_stage() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);
        let wilt = create_test_mesh(4);
        let aging = FlowerAging::with_wilt(bud, bloom, wilt);

        // Old flowers should get wilt mesh
        assert_eq!(aging.select_mesh(0.8).vertex_count(), 4);
        assert_eq!(aging.select_mesh(0.9).vertex_count(), 4);
        assert_eq!(aging.select_mesh(1.0).vertex_count(), 4);
    }

    #[test]
    fn test_select_mesh_wilt_fallback() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);
        let aging = FlowerAging::new(bud, bloom);

        // Without wilt mesh, should fallback to bloom
        assert_eq!(aging.select_mesh(0.8).vertex_count(), 5);
        assert_eq!(aging.select_mesh(1.0).vertex_count(), 5);
    }

    #[test]
    fn test_thresholds() {
        let aging = FlowerAging::new(create_test_mesh(1), create_test_mesh(1));
        let (bud_threshold, wilt_threshold) = aging.thresholds();

        assert_eq!(bud_threshold, 0.3);
        assert_eq!(wilt_threshold, 0.8);
    }

    #[test]
    fn test_age_boundary_conditions() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);
        let wilt = create_test_mesh(4);
        let aging = FlowerAging::with_wilt(bud, bloom, wilt);

        // Test exact boundaries
        assert_eq!(
            aging.select_mesh(0.0).vertex_count(),
            3,
            "age=0.0 should be bud"
        );
        assert_eq!(
            aging.select_mesh(0.3).vertex_count(),
            5,
            "age=0.3 should be bloom"
        );
        assert_eq!(
            aging.select_mesh(0.8).vertex_count(),
            4,
            "age=0.8 should be wilt"
        );
        assert_eq!(
            aging.select_mesh(1.0).vertex_count(),
            4,
            "age=1.0 should be wilt"
        );
    }

    #[test]
    fn test_age_progression() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);
        let wilt = create_test_mesh(4);
        let aging = FlowerAging::with_wilt(bud, bloom, wilt);

        // Simulate aging from 0.0 to 1.0
        let ages = [0.0, 0.2, 0.3, 0.5, 0.7, 0.8, 0.9, 1.0];
        let expected_vertices = [3, 3, 5, 5, 5, 4, 4, 4];

        for (age, expected) in ages.iter().zip(expected_vertices.iter()) {
            let mesh = aging.select_mesh(*age);
            assert_eq!(
                mesh.vertex_count(),
                *expected,
                "age={} should have {} vertices",
                age,
                expected
            );
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serialization() {
        let bud = create_test_mesh(3);
        let bloom = create_test_mesh(5);
        let aging = FlowerAging::new(bud, bloom);

        let json = serde_json::to_string(&aging).unwrap();
        let deserialized: FlowerAging = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.bud_mesh.vertex_count(), 3);
        assert_eq!(deserialized.bloom_mesh.vertex_count(), 5);
    }
}
