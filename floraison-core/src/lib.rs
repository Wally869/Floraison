//! Floraison Core
//!
//! Core mathematical and geometric primitives for procedural flower generation.
//! This crate provides the foundational building blocks used by higher-level
//! flower component generators.

// Re-export glam types for convenience
pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

pub mod math {
    //! Mathematical utilities for flower generation

    /// Vector and matrix utilities
    pub mod vector;

    /// Phyllotaxis (spiral arrangements) calculations
    pub mod phyllotaxis;

    /// Bézier curve evaluation
    pub mod bezier;

    /// B-spline curves and surfaces
    pub mod bspline;

    /// 3D curve utilities (Catmull-Rom splines, etc.)
    pub mod curves;
}

pub mod geometry {
    //! Geometric primitives and mesh generation

    /// Mesh data structures
    pub mod mesh;

    /// Surface of revolution generator
    pub mod surface_revolution;

    /// Sweep surface generator (extrude profile along curve)
    pub mod sweep;

    pub mod tessellation {
        //! Mesh tessellation and subdivision
        // Will be implemented as needed
    }
}
