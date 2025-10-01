//! Inflorescence generation for procedural flowers
//!
//! This module provides data structures and pattern generators for creating
//! inflorescences (multi-flower structures) with various botanical arrangements.
//!
//! Inflorescences are classified into two main categories:
//! - **Indeterminate**: Lower/outer flowers bloom first (raceme, spike, umbel, corymb)
//! - **Determinate**: Upper/central flowers bloom first (dichasium, drepanium)
//! - **Compound**: Recursive combinations of the above patterns

use floraison_core::Vec3;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod patterns;
pub mod assembly;

/// Inflorescence pattern type
///
/// Defines the branching and arrangement pattern for multi-flower structures.
///
/// # Pattern Categories
///
/// ## Indeterminate (Acropetal)
/// Flowers bloom from bottom to top or outside to inside:
/// - [`Raceme`](PatternType::Raceme): Flowers on pedicels along main axis
/// - [`Spike`](PatternType::Spike): Flowers sessile (no pedicels) on main axis
/// - [`Umbel`](PatternType::Umbel): All pedicels from single point (umbrella-like)
/// - [`Corymb`](PatternType::Corymb): Varied pedicel lengths creating flat top
///
/// ## Determinate (Basipetal)
/// Central/terminal flower blooms first:
/// - [`Dichasium`](PatternType::Dichasium): Two branches per node (Y-shaped)
/// - [`Drepanium`](PatternType::Drepanium): Single branch per node, spiraling
///
/// ## Compound
/// Recursive patterns where each branch ends in a sub-pattern:
/// - [`CompoundRaceme`](PatternType::CompoundRaceme): Raceme of racemes
/// - [`CompoundUmbel`](PatternType::CompoundUmbel): Umbel of umbels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PatternType {
    /// Raceme: Flowers on pedicels along unbranched axis
    ///
    /// Example: Snapdragon, Lupine
    Raceme,

    /// Spike: Flowers sessile (no pedicels) on unbranched axis
    ///
    /// Example: Wheat, Plantain
    Spike,

    /// Umbel: All pedicels originate from single point
    ///
    /// Example: Onion, Carrot
    Umbel,

    /// Corymb: Pedicels of varying length create flat-topped appearance
    ///
    /// Example: Hawthorn, Yarrow
    Corymb,

    /// Dichasium: Two opposite branches at each node
    ///
    /// Example: Many carnations
    Dichasium,

    /// Drepanium: Single branch per node, all on same side (scorpioid cyme)
    ///
    /// Example: Forget-me-not
    Drepanium,

    /// Compound Raceme: Raceme where each flower is replaced by sub-raceme
    ///
    /// Example: Lilac, Astilbe
    CompoundRaceme,

    /// Compound Umbel: Umbel where each ray ends in sub-umbel
    ///
    /// Example: Parsley, Dill
    CompoundUmbel,
}

/// Parameters defining an inflorescence structure
///
/// Controls the overall shape and arrangement of flowers along the main axis.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InflorescenceParams {
    /// Pattern type (raceme, spike, umbel, etc.)
    pub pattern: PatternType,

    /// Length of main axis
    pub axis_length: f32,

    /// Number of branch points (flower positions)
    pub branch_count: usize,

    /// Branch angle at top of inflorescence (degrees from vertical)
    ///
    /// 0° = upright, 90° = horizontal
    pub angle_top: f32,

    /// Branch angle at bottom of inflorescence (degrees from vertical)
    pub angle_bottom: f32,

    /// Branch (pedicel) length at top
    pub branch_length_top: f32,

    /// Branch (pedicel) length at bottom
    pub branch_length_bottom: f32,

    /// Rotation angle between successive branches (degrees)
    ///
    /// Common values:
    /// - 360/n for even spacing
    /// - 137.5° (golden angle) for natural spiral
    /// - 180° for opposite arrangement
    pub rotation_angle: f32,

    /// Flower scale factor at top
    pub flower_size_top: f32,

    /// Flower scale factor at bottom
    pub flower_size_bottom: f32,
}

impl Default for InflorescenceParams {
    fn default() -> Self {
        Self {
            pattern: PatternType::Raceme,
            axis_length: 10.0,
            branch_count: 12,
            angle_top: 45.0,
            angle_bottom: 60.0,
            branch_length_top: 0.5,
            branch_length_bottom: 1.5,
            rotation_angle: 137.5, // Golden angle
            flower_size_top: 0.8,
            flower_size_bottom: 1.0,
        }
    }
}

/// Represents a single branch point where a flower attaches
///
/// Contains all information needed to position and orient a flower
/// at a specific location on the inflorescence.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BranchPoint {
    /// 3D position of flower base
    pub position: Vec3,

    /// Direction vector the flower faces
    ///
    /// Normalized vector indicating flower orientation.
    /// For pedicels, this is the direction from axis to flower.
    pub direction: Vec3,

    /// Length of pedicel (branch stem)
    ///
    /// 0.0 for sessile flowers (spike pattern)
    pub length: f32,

    /// Scale factor for flower size
    ///
    /// 1.0 = normal size, < 1.0 = smaller, > 1.0 = larger
    pub flower_scale: f32,

    /// Developmental age of flower
    ///
    /// 0.0 = bud, 1.0 = full bloom
    ///
    /// For indeterminate patterns: bottom (oldest) = 1.0, top (youngest) = 0.0
    /// For determinate patterns: center/top (oldest) = 1.0, outer/bottom (youngest) = 0.0
    pub age: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_type_equality() {
        assert_eq!(PatternType::Raceme, PatternType::Raceme);
        assert_ne!(PatternType::Raceme, PatternType::Spike);
    }

    #[test]
    fn test_default_params() {
        let params = InflorescenceParams::default();
        assert_eq!(params.pattern, PatternType::Raceme);
        assert_eq!(params.branch_count, 12);
        assert_eq!(params.rotation_angle, 137.5);
    }

    #[test]
    fn test_branch_point_creation() {
        let branch = BranchPoint {
            position: Vec3::new(1.0, 2.0, 3.0),
            direction: Vec3::Y,
            length: 1.5,
            flower_scale: 1.0,
            age: 0.5,
        };

        assert!((branch.position - Vec3::new(1.0, 2.0, 3.0)).length() < 1e-5);
        assert!((branch.direction - Vec3::Y).length() < 1e-5);
        assert_eq!(branch.length, 1.5);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serialization() {
        let params = InflorescenceParams::default();
        let json = serde_json::to_string(&params).unwrap();
        let deserialized: InflorescenceParams = serde_json::from_str(&json).unwrap();

        assert_eq!(params.pattern, deserialized.pattern);
        assert_eq!(params.branch_count, deserialized.branch_count);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_pattern_type_serde() {
        let pattern = PatternType::Umbel;
        let json = serde_json::to_string(&pattern).unwrap();
        let deserialized: PatternType = serde_json::from_str(&json).unwrap();

        assert_eq!(pattern, deserialized);
    }
}
