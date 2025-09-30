//! Floral diagram and component arrangement
//!
//! The floral diagram defines the spatial arrangement pattern for flower components.
//! It specifies how many of each component type to create and where to position them.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Arrangement pattern for components in a whorl (concentric ring)
///
/// Defines how components are distributed around a circle.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArrangementPattern {
    /// Components evenly distributed around the circle
    /// Angle between components = 360° / count
    EvenlySpaced,

    /// Components arranged in a spiral pattern using golden angle
    /// Creates natural-looking phyllotaxis (leaf arrangement)
    GoldenSpiral,

    /// Components arranged with specified angular offset between each
    /// Useful for creating alternating patterns
    CustomOffset(f32),
}

/// Definition of a component whorl (concentric ring of components)
///
/// A whorl represents a ring of similar components at a specific radius
/// from the flower center.
///
/// # Example
/// ```
/// use floraison_components::diagram::{ComponentWhorl, ArrangementPattern};
///
/// // Create a whorl of 6 petals at radius 1.0
/// let petal_whorl = ComponentWhorl {
///     count: 6,
///     radius: 1.0,
///     height: 0.5,
///     pattern: ArrangementPattern::EvenlySpaced,
///     rotation_offset: 0.0,
/// };
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComponentWhorl {
    /// Number of components in this whorl
    pub count: usize,

    /// Radial distance from flower center
    pub radius: f32,

    /// Vertical height (Y coordinate) where components attach
    pub height: f32,

    /// Angular arrangement pattern
    pub pattern: ArrangementPattern,

    /// Initial rotation offset in radians
    /// Allows rotating the entire whorl
    pub rotation_offset: f32,
}

impl ComponentWhorl {
    /// Calculate the angular positions for all components in this whorl
    ///
    /// Returns a vector of angles in radians, one for each component.
    pub fn calculate_angles(&self) -> Vec<f32> {
        let mut angles = Vec::with_capacity(self.count);

        match self.pattern {
            ArrangementPattern::EvenlySpaced => {
                let angle_step = std::f32::consts::TAU / self.count as f32;
                for i in 0..self.count {
                    angles.push(self.rotation_offset + i as f32 * angle_step);
                }
            }
            ArrangementPattern::GoldenSpiral => {
                // Golden angle ≈ 137.5° in radians
                const GOLDEN_ANGLE: f32 = 2.399963; // (3 - sqrt(5)) * π
                for i in 0..self.count {
                    angles.push(self.rotation_offset + i as f32 * GOLDEN_ANGLE);
                }
            }
            ArrangementPattern::CustomOffset(offset) => {
                for i in 0..self.count {
                    angles.push(self.rotation_offset + i as f32 * offset);
                }
            }
        }

        angles
    }
}

/// Complete floral diagram defining all component arrangements
///
/// This structure specifies the complete spatial arrangement for a flower,
/// including receptacle parameters and whorls for each component type.
///
/// # Example
/// ```
/// use floraison_components::diagram::FloralDiagram;
///
/// // Create a simple lily-like flower
/// let diagram = FloralDiagram::lily();
/// assert_eq!(diagram.petal_whorls.len(), 1);
/// assert_eq!(diagram.petal_whorls[0].count, 6);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FloralDiagram {
    /// Height of the receptacle (flower base)
    pub receptacle_height: f32,

    /// Radius of the receptacle at its base
    pub receptacle_radius: f32,

    /// Whorl(s) of petals (usually 1 whorl, but can have multiple)
    pub petal_whorls: Vec<ComponentWhorl>,

    /// Whorl(s) of stamens (male reproductive structures)
    pub stamen_whorls: Vec<ComponentWhorl>,

    /// Whorl(s) of pistils (female reproductive structures)
    /// Usually just one pistil at the center, but some flowers have multiple
    pub pistil_whorls: Vec<ComponentWhorl>,

    /// Optional whorl(s) of sepals (outer protective leaves)
    /// Will be implemented in future epics
    pub sepal_whorls: Vec<ComponentWhorl>,
}

impl FloralDiagram {
    /// Create a lily-like flower diagram
    ///
    /// Characteristics:
    /// - 6 petals in one whorl
    /// - 6 stamens in one whorl (alternating with petals)
    /// - 1 central pistil
    pub fn lily() -> Self {
        Self {
            receptacle_height: 1.0,
            receptacle_radius: 0.3,
            petal_whorls: vec![ComponentWhorl {
                count: 6,
                radius: 1.0,
                height: 0.8,
                pattern: ArrangementPattern::EvenlySpaced,
                rotation_offset: 0.0,
            }],
            stamen_whorls: vec![ComponentWhorl {
                count: 6,
                radius: 0.6,
                height: 0.6,
                pattern: ArrangementPattern::EvenlySpaced,
                rotation_offset: std::f32::consts::PI / 6.0, // Offset by 30° to alternate
            }],
            pistil_whorls: vec![ComponentWhorl {
                count: 1,
                radius: 0.0,
                height: 0.5,
                pattern: ArrangementPattern::EvenlySpaced,
                rotation_offset: 0.0,
            }],
            sepal_whorls: vec![],
        }
    }

    /// Create a simple 5-petal flower diagram (like a rose or apple blossom)
    ///
    /// Characteristics:
    /// - 5 petals evenly spaced
    /// - 10 stamens in two whorls
    /// - 1 central pistil
    pub fn five_petal() -> Self {
        Self {
            receptacle_height: 0.8,
            receptacle_radius: 0.4,
            petal_whorls: vec![ComponentWhorl {
                count: 5,
                radius: 1.2,
                height: 0.6,
                pattern: ArrangementPattern::EvenlySpaced,
                rotation_offset: 0.0,
            }],
            stamen_whorls: vec![
                ComponentWhorl {
                    count: 5,
                    radius: 0.7,
                    height: 0.5,
                    pattern: ArrangementPattern::EvenlySpaced,
                    rotation_offset: 0.0,
                },
                ComponentWhorl {
                    count: 5,
                    radius: 0.5,
                    height: 0.4,
                    pattern: ArrangementPattern::EvenlySpaced,
                    rotation_offset: std::f32::consts::PI / 5.0,
                },
            ],
            pistil_whorls: vec![ComponentWhorl {
                count: 1,
                radius: 0.0,
                height: 0.3,
                pattern: ArrangementPattern::EvenlySpaced,
                rotation_offset: 0.0,
            }],
            sepal_whorls: vec![],
        }
    }

    /// Create a daisy-like flower diagram
    ///
    /// Characteristics:
    /// - Many petals (13-21) in spiral arrangement
    /// - Many stamens in spiral at center
    /// - Multiple pistils in spiral
    pub fn daisy() -> Self {
        Self {
            receptacle_height: 0.5,
            receptacle_radius: 0.8,
            petal_whorls: vec![ComponentWhorl {
                count: 21,
                radius: 1.5,
                height: 0.4,
                pattern: ArrangementPattern::GoldenSpiral,
                rotation_offset: 0.0,
            }],
            stamen_whorls: vec![ComponentWhorl {
                count: 34,
                radius: 0.7,
                height: 0.3,
                pattern: ArrangementPattern::GoldenSpiral,
                rotation_offset: 0.5,
            }],
            pistil_whorls: vec![ComponentWhorl {
                count: 13,
                radius: 0.4,
                height: 0.2,
                pattern: ArrangementPattern::GoldenSpiral,
                rotation_offset: 1.0,
            }],
            sepal_whorls: vec![],
        }
    }

    /// Create a simple 4-petal flower diagram
    ///
    /// Characteristics:
    /// - 4 petals in cross pattern
    /// - 4 stamens alternating with petals
    /// - 1 central pistil
    pub fn four_petal() -> Self {
        Self {
            receptacle_height: 0.6,
            receptacle_radius: 0.3,
            petal_whorls: vec![ComponentWhorl {
                count: 4,
                radius: 1.0,
                height: 0.5,
                pattern: ArrangementPattern::EvenlySpaced,
                rotation_offset: std::f32::consts::PI / 4.0, // 45° offset for cross pattern
            }],
            stamen_whorls: vec![ComponentWhorl {
                count: 4,
                radius: 0.5,
                height: 0.4,
                pattern: ArrangementPattern::EvenlySpaced,
                rotation_offset: 0.0,
            }],
            pistil_whorls: vec![ComponentWhorl {
                count: 1,
                radius: 0.0,
                height: 0.3,
                pattern: ArrangementPattern::EvenlySpaced,
                rotation_offset: 0.0,
            }],
            sepal_whorls: vec![],
        }
    }

    /// Get the total number of petals in this diagram
    pub fn total_petal_count(&self) -> usize {
        self.petal_whorls.iter().map(|w| w.count).sum()
    }

    /// Get the total number of stamens in this diagram
    pub fn total_stamen_count(&self) -> usize {
        self.stamen_whorls.iter().map(|w| w.count).sum()
    }

    /// Get the total number of pistils in this diagram
    pub fn total_pistil_count(&self) -> usize {
        self.pistil_whorls.iter().map(|w| w.count).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evenly_spaced_angles() {
        let whorl = ComponentWhorl {
            count: 6,
            radius: 1.0,
            height: 0.5,
            pattern: ArrangementPattern::EvenlySpaced,
            rotation_offset: 0.0,
        };

        let angles = whorl.calculate_angles();
        assert_eq!(angles.len(), 6);

        // Check angles are evenly spaced (60° apart)
        let expected_step = std::f32::consts::TAU / 6.0;
        for i in 0..6 {
            let expected = i as f32 * expected_step;
            assert!((angles[i] - expected).abs() < 0.001);
        }
    }

    #[test]
    fn test_golden_spiral_angles() {
        let whorl = ComponentWhorl {
            count: 5,
            radius: 1.0,
            height: 0.5,
            pattern: ArrangementPattern::GoldenSpiral,
            rotation_offset: 0.0,
        };

        let angles = whorl.calculate_angles();
        assert_eq!(angles.len(), 5);

        // Check that angles follow golden spiral
        const GOLDEN_ANGLE: f32 = 2.399963;
        for i in 0..5 {
            let expected = i as f32 * GOLDEN_ANGLE;
            assert!((angles[i] - expected).abs() < 0.001);
        }
    }

    #[test]
    fn test_rotation_offset() {
        let whorl = ComponentWhorl {
            count: 4,
            radius: 1.0,
            height: 0.5,
            pattern: ArrangementPattern::EvenlySpaced,
            rotation_offset: std::f32::consts::PI / 4.0,
        };

        let angles = whorl.calculate_angles();
        assert_eq!(angles.len(), 4);

        // First angle should be the rotation offset
        assert!((angles[0] - std::f32::consts::PI / 4.0).abs() < 0.001);
    }

    #[test]
    fn test_lily_diagram() {
        let diagram = FloralDiagram::lily();

        assert_eq!(diagram.total_petal_count(), 6);
        assert_eq!(diagram.total_stamen_count(), 6);
        assert_eq!(diagram.total_pistil_count(), 1);

        assert_eq!(diagram.petal_whorls.len(), 1);
        assert_eq!(diagram.stamen_whorls.len(), 1);
        assert_eq!(diagram.pistil_whorls.len(), 1);
    }

    #[test]
    fn test_five_petal_diagram() {
        let diagram = FloralDiagram::five_petal();

        assert_eq!(diagram.total_petal_count(), 5);
        assert_eq!(diagram.total_stamen_count(), 10); // Two whorls of 5
        assert_eq!(diagram.total_pistil_count(), 1);

        assert_eq!(diagram.stamen_whorls.len(), 2);
    }

    #[test]
    fn test_daisy_diagram() {
        let diagram = FloralDiagram::daisy();

        assert_eq!(diagram.total_petal_count(), 21);
        assert_eq!(diagram.total_stamen_count(), 34);
        assert_eq!(diagram.total_pistil_count(), 13);

        // Check that daisy uses golden spiral
        assert_eq!(diagram.petal_whorls[0].pattern, ArrangementPattern::GoldenSpiral);
        assert_eq!(diagram.stamen_whorls[0].pattern, ArrangementPattern::GoldenSpiral);
        assert_eq!(diagram.pistil_whorls[0].pattern, ArrangementPattern::GoldenSpiral);
    }

    #[test]
    fn test_four_petal_diagram() {
        let diagram = FloralDiagram::four_petal();

        assert_eq!(diagram.total_petal_count(), 4);
        assert_eq!(diagram.total_stamen_count(), 4);
        assert_eq!(diagram.total_pistil_count(), 1);
    }

    #[test]
    fn test_custom_offset_pattern() {
        let whorl = ComponentWhorl {
            count: 3,
            radius: 1.0,
            height: 0.5,
            pattern: ArrangementPattern::CustomOffset(1.0),
            rotation_offset: 0.0,
        };

        let angles = whorl.calculate_angles();
        assert_eq!(angles.len(), 3);

        // Check angles increment by custom offset
        for i in 0..3 {
            let expected = i as f32 * 1.0;
            assert!((angles[i] - expected).abs() < 0.001);
        }
    }

    #[test]
    fn test_whorl_angles_non_negative() {
        let diagram = FloralDiagram::lily();

        for whorl in &diagram.petal_whorls {
            let angles = whorl.calculate_angles();
            for angle in angles {
                assert!(angle >= 0.0, "Angle should be non-negative");
            }
        }
    }
}
