//! Floraison Components
//!
//! Generators for individual floral components (receptacle, pistil, stamen, petal, sepal).
//! Each component is parameterized and generates geometry that can be assembled into
//! complete flowers.

pub use floraison_core::{Vec2, Vec3};

/// Receptacle (base of flower) generator
pub mod receptacle {
    //! Receptacle generator using surface of revolution
    // Will be implemented in Task 3.1
}

/// Pistil (female reproductive structure) generator
pub mod pistil {
    //! Pistil generator with style and stigma
    // Will be implemented in Task 3.2
}

/// Stamen (male reproductive structure) generator
pub mod stamen {
    //! Stamen generator with filament and anther
    // Will be implemented in Task 3.3
}

/// Petal generator
pub mod petal {
    //! Petal generator with B-spline surfaces
    // Will be implemented in Task 3.4 (simple) and Task 6.5 (B-spline)
}

/// Sepal generator
pub mod sepal {
    //! Sepal generator (reuses petal logic)
    // Will be implemented in Task 7.1
}
