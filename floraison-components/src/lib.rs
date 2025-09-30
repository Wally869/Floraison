//! Floraison Components
//!
//! Generators for individual floral components (receptacle, pistil, stamen, petal, sepal).
//! Each component is parameterized and generates geometry that can be assembled into
//! complete flowers.

pub use floraison_core::{Vec2, Vec3, Mat4, Quat};
pub use floraison_core::geometry::mesh::Mesh;

/// Receptacle (base of flower) generator
pub mod receptacle;

/// Pistil (female reproductive structure) generator
pub mod pistil;

/// Stamen (male reproductive structure) generator
pub mod stamen;

/// Petal generator
pub mod petal;

/// Floral diagram and component arrangement
pub mod diagram;

/// Flower assembly and component positioning
pub mod assembly;

/// Sepal generator (placeholder)
pub mod sepal {
    //! Sepal generator (reuses petal logic)
    // Will be implemented in Task 7.1
}
