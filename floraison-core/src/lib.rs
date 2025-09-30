//! Floraison Core
//!
//! Core mathematical and geometric primitives for procedural flower generation.
//! This crate provides the foundational building blocks used by higher-level
//! flower component generators.

// Re-export glam types for convenience
pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

/// Mathematical utilities for flower generation
pub mod math {
    //! Mathematical utilities for flower generation

    /// Vector and matrix utilities
    pub mod vector;

    /// Phyllotaxis (spiral arrangements) calculations
    pub mod phyllotaxis;

    /// Bézier curve evaluation (placeholder)
    pub mod bezier {
        //! Bézier curve utilities for smooth profiles
        // Will be implemented in Task 2.5
    }

    /// B-spline curves and surfaces (placeholder)
    pub mod bspline {
        //! B-spline surface evaluation for petals
        // Will be implemented in Task 6.1
    }

    /// 3D curve reconstruction (placeholder)
    pub mod curves {
        //! 3D curve reconstruction from 2D input
        // Will be implemented in Task 10.2
    }
}

/// Geometric primitives and mesh generation
pub mod geometry {
    //! Geometric primitives and mesh generation

    /// Mesh data structures
    pub mod mesh;

    /// Surface of revolution generator
    pub mod surface_revolution;

    /// Tessellation utilities (placeholder)
    pub mod tessellation {
        //! Mesh tessellation and subdivision
        // Will be implemented as needed
    }
}
