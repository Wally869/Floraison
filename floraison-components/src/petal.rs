//! Petal generator
//!
//! Creates petals with B-spline surfaces supporting deformations (curl, twist, ruffle).

use crate::{Mesh, Vec2, Vec3};
use floraison_core::math::bezier::sample_cubic_2d;
use floraison_core::math::bspline::{generate_knot_vector, BSplineSurface};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Parameters for petal generation
///
/// # Example
/// ```
/// use floraison_components::petal::{PetalParams, generate};
/// use floraison_components::Vec3;
///
/// let params = PetalParams {
///     length: 3.0,
///     width: 1.5,
///     tip_sharpness: 0.3,
///     base_width: 0.5,
///     curl: 0.0,
///     twist: 0.0,
///     lateral_curve: 0.0,
///     ruffle_freq: 0.0,
///     ruffle_amp: 0.0,
///     resolution: 16,
///     color: Vec3::ONE,
/// };
///
/// let mesh = generate(&params);
/// assert!(mesh.vertex_count() > 0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PetalParams {
    /// Length of the petal (from base to tip)
    pub length: f32,

    /// Maximum width of the petal
    pub width: f32,

    /// Tip sharpness (0.0 = rounded, 1.0 = very pointed)
    /// Controls how much the tip curves inward
    pub tip_sharpness: f32,

    /// Width at the base of the petal
    pub base_width: f32,

    /// Curl amount (-1.0 = curl down, 0.0 = flat, 1.0 = curl up)
    pub curl: f32,

    /// Twist angle in degrees (applied progressively toward tip)
    pub twist: f32,

    /// Ruffle frequency (number of waves along edges)
    pub ruffle_freq: f32,

    /// Ruffle amplitude (height of edge waves)
    pub ruffle_amp: f32,

    /// Lateral curve amount (-1.0 = curve left, 0.0 = straight, 1.0 = curve right)
    /// Bends the petal sideways in the XY plane
    pub lateral_curve: f32,

    /// Tessellation resolution (samples per parametric direction)
    pub resolution: usize,

    /// RGB color in 0.0-1.0 range
    pub color: Vec3,
}

impl Default for PetalParams {
    /// Create default parameters for a lily-like petal
    fn default() -> Self {
        Self {
            length: 3.0,
            width: 1.2,
            tip_sharpness: 0.4,
            base_width: 0.4,
            curl: 0.0,
            twist: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            lateral_curve: 0.0,
            resolution: 16,
            color: Vec3::ONE, // White
        }
    }
}

impl PetalParams {
    /// Create a wide, rounded petal
    pub fn wide() -> Self {
        Self {
            length: 2.5,
            width: 2.0,
            tip_sharpness: 0.2,
            base_width: 0.8,
            curl: 0.0,
            twist: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            lateral_curve: 0.0,
            resolution: 20,
            color: Vec3::ONE,
        }
    }

    /// Create a narrow, pointed petal
    pub fn narrow() -> Self {
        Self {
            length: 4.0,
            width: 1.0,
            tip_sharpness: 0.7,
            base_width: 0.3,
            curl: 0.0,
            twist: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            lateral_curve: 0.0,
            resolution: 16,
            color: Vec3::ONE,
        }
    }

    /// Create a short, rounded petal
    pub fn short() -> Self {
        Self {
            length: 1.5,
            width: 1.2,
            tip_sharpness: 0.1,
            base_width: 0.6,
            curl: 0.0,
            twist: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            lateral_curve: 0.0,
            resolution: 12,
            color: Vec3::ONE,
        }
    }
}

/// Generate a 2D control point grid for B-spline petal surface
///
/// Creates a rectangular grid of control points that matches the petal outline shape:
/// - Narrow at base
/// - Wide in middle
/// - Tapered at tip
///
/// The grid is oriented in the XY plane (all Z = 0) with the base at y=0
/// and tip at y=length.
///
/// # Grid Dimensions
///
/// - **Rows** (v direction, along length): 9 points
/// - **Columns** (u direction, across width): 5 points
///
/// This provides smooth B-spline surfaces with cubic degree (p=3).
///
/// # Arguments
///
/// * `params` - Petal parameters defining dimensions
///
/// # Returns
///
/// 2D vector of control points: `grid[row][col]` where:
/// - `row` index corresponds to v parameter (length direction)
/// - `col` index corresponds to u parameter (width direction)
///
/// # Example
///
/// ```
/// use floraison_components::petal::{PetalParams, generate_control_grid};
///
/// let params = PetalParams::default();
/// let grid = generate_control_grid(&params);
///
/// assert_eq!(grid.len(), 9);  // 9 rows
/// assert_eq!(grid[0].len(), 5);  // 5 columns
/// ```
pub fn generate_control_grid(params: &PetalParams) -> Vec<Vec<Vec3>> {
    const ROWS: usize = 9; // Along length (v direction)
    const COLS: usize = 5; // Across width (u direction)

    let mut grid = vec![vec![Vec3::ZERO; COLS]; ROWS];

    for (row, row_data) in grid.iter_mut().enumerate().take(ROWS) {
        // v parameter: 0.0 at base, 1.0 at tip
        let v = row as f32 / (ROWS - 1) as f32;
        let y = v * params.length;

        // Interpolate width along length to match petal outline
        // Base -> middle: narrow to wide
        // Middle -> tip: wide to tapered
        let width_at_v = if v < 0.6 {
            // 0 to 60% of length: interpolate from base_width to full width
            let t = v / 0.6;
            params.base_width + (params.width - params.base_width) * t
        } else {
            // 60% to 100%: interpolate from full width to tip width
            let t = (v - 0.6) / 0.4;
            params.width + (params.width * params.tip_sharpness - params.width) * t
        };

        for (col, cell) in row_data.iter_mut().enumerate().take(COLS) {
            // u parameter: 0.0 at left edge, 1.0 at right edge
            let u = col as f32 / (COLS - 1) as f32;

            // Map u to x coordinate: centered at x=0
            let x = (u - 0.5) * width_at_v;

            *cell = Vec3::new(x, y, 0.0);
        }
    }

    grid
}

/// Apply curl deformation to control points
///
/// Curls the petal by rotating points around a horizontal axis (bending in YZ plane).
/// The curl increases along the length of the petal.
///
/// # Arguments
///
/// * `control_points` - Mutable reference to 2D grid of control points
/// * `amount` - Curl amount in range [-1, 1]:
///   - Negative values curl downward
///   - Positive values curl upward
///   - 0 = no curl
///
/// # Example
///
/// ```
/// use floraison_components::petal::{PetalParams, generate_control_grid, apply_curl};
///
/// let params = PetalParams::default();
/// let mut grid = generate_control_grid(&params);
///
/// // Curl upward
/// apply_curl(&mut grid, 0.5);
/// ```
pub fn apply_curl(control_points: &mut [Vec<Vec3>], amount: f32) {
    use std::f32::consts::PI;

    let rows = control_points.len();

    for (row_idx, row) in control_points.iter_mut().enumerate() {
        // v parameter: 0.0 at base, 1.0 at tip
        let v = row_idx as f32 / (rows - 1) as f32;

        // Curl increases quadratically along length for smooth deformation
        let curl_factor = v * v;
        let curl_angle = amount * curl_factor * PI * 0.5;

        for point in row.iter_mut() {
            let y = point.y;
            let z = point.z;

            // Rotate in YZ plane around X axis
            // Positive angle curls upward (toward +Z)
            point.y = y * curl_angle.cos() - z * curl_angle.sin();
            point.z = y * curl_angle.sin() + z * curl_angle.cos();
        }
    }
}

/// Apply twist deformation to control points
///
/// Twists the petal around its central axis (Y axis).
/// The twist increases toward the tip.
///
/// # Arguments
///
/// * `control_points` - Mutable reference to 2D grid of control points
/// * `angle_deg` - Total twist angle in degrees at the tip
///   - Positive = counter-clockwise twist when viewed from tip
///   - Negative = clockwise twist
///   - 0 = no twist
///
/// # Example
///
/// ```
/// use floraison_components::petal::{PetalParams, generate_control_grid, apply_twist};
///
/// let params = PetalParams::default();
/// let mut grid = generate_control_grid(&params);
///
/// // Twist 45 degrees counter-clockwise
/// apply_twist(&mut grid, 45.0);
/// ```
pub fn apply_twist(control_points: &mut [Vec<Vec3>], angle_deg: f32) {
    let angle_rad = angle_deg.to_radians();
    let rows = control_points.len();

    for (row_idx, row) in control_points.iter_mut().enumerate() {
        // v parameter: 0.0 at base, 1.0 at tip
        let v = row_idx as f32 / (rows - 1) as f32;

        // Twist increases linearly toward tip
        let twist_angle = angle_rad * v;

        for point in row.iter_mut() {
            let x = point.x;
            let z = point.z;

            // Rotate in XZ plane around Y axis
            point.x = x * twist_angle.cos() - z * twist_angle.sin();
            point.z = x * twist_angle.sin() + z * twist_angle.cos();
        }
    }
}

/// Apply lateral curve deformation to control points
///
/// Curves the petal sideways by rotating points in the XY plane.
/// The curve increases along the length of the petal.
///
/// # Arguments
///
/// * `control_points` - Mutable reference to 2D grid of control points
/// * `amount` - Lateral curve amount in range [-1, 1]:
///   - Negative values curve left
///   - Positive values curve right
///   - 0 = no lateral curve
///
/// # Example
///
/// ```
/// use floraison_components::petal::{PetalParams, generate_control_grid, apply_lateral_curve};
///
/// let params = PetalParams::default();
/// let mut grid = generate_control_grid(&params);
///
/// // Curve to the right
/// apply_lateral_curve(&mut grid, 0.5);
/// ```
pub fn apply_lateral_curve(control_points: &mut [Vec<Vec3>], amount: f32) {
    use std::f32::consts::PI;

    let rows = control_points.len();

    for (row_idx, row) in control_points.iter_mut().enumerate() {
        // v parameter: 0.0 at base, 1.0 at tip
        let v = row_idx as f32 / (rows - 1) as f32;

        // Curve increases quadratically along length for smooth deformation
        let curve_factor = v * v;
        // Use a smaller multiplier than curl for more subtle lateral bending
        let curve_angle = amount * curve_factor * PI * 0.3;

        for point in row.iter_mut() {
            let x = point.x;
            let y = point.y;

            // Rotate in XY plane around Z axis
            // Positive angle curves right, negative curves left
            point.x = x * curve_angle.cos() - y * curve_angle.sin();
            point.y = x * curve_angle.sin() + y * curve_angle.cos();
        }
    }
}

/// Apply ruffle deformation to control points
///
/// Adds sinusoidal waves to the edges of the petal for a ruffled appearance.
/// Only affects points near the edges (high u or low u values).
///
/// # Arguments
///
/// * `control_points` - Mutable reference to 2D grid of control points
/// * `frequency` - Number of waves along the petal length
/// * `amplitude` - Height of the waves
///
/// # Example
///
/// ```
/// use floraison_components::petal::{PetalParams, generate_control_grid, apply_ruffle};
///
/// let params = PetalParams::default();
/// let mut grid = generate_control_grid(&params);
///
/// // Add 3 waves with amplitude 0.2
/// apply_ruffle(&mut grid, 3.0, 0.2);
/// ```
pub fn apply_ruffle(control_points: &mut [Vec<Vec3>], frequency: f32, amplitude: f32) {
    use std::f32::consts::PI;

    let rows = control_points.len();
    let cols = control_points[0].len();

    for (row_idx, row) in control_points.iter_mut().enumerate() {
        // v parameter: 0.0 at base, 1.0 at tip
        let v = row_idx as f32 / (rows - 1) as f32;

        // Wave varies along length
        let wave_phase = v * frequency * PI * 2.0;
        let wave_value = wave_phase.sin();

        for (col_idx, point) in row.iter_mut().enumerate() {
            // u parameter: 0.0 at left edge, 1.0 at right edge
            let u = col_idx as f32 / (cols - 1) as f32;

            // Edge weight: 0 at center, 1 at edges
            let edge_weight = if u < 0.5 {
                // Left edge: closer to 0, higher weight
                1.0 - (u * 2.0)
            } else {
                // Right edge: closer to 1, higher weight
                (u - 0.5) * 2.0
            };

            // Only apply ruffle to edges (when edge_weight > 0.3)
            if edge_weight > 0.3 {
                // Add wave displacement in Z direction
                point.z += wave_value * amplitude * edge_weight;
            }
        }
    }
}

/// Generate a petal mesh using B-spline surfaces with deformations
///
/// Creates a 3D petal using B-spline surface evaluation with support for:
/// - Curl (bending up/down)
/// - Twist (rotating around center)
/// - Ruffle (wavy edges)
///
/// The petal is generated by:
/// 1. Creating a control point grid matching the outline shape
/// 2. Applying deformations (curl, twist, ruffle)
/// 3. Creating a B-spline surface
/// 4. Tessellating the surface at the specified resolution
/// 5. Adding back faces for double-sided rendering
///
/// # Arguments
///
/// * `params` - Petal parameters
///
/// # Returns
///
/// A mesh with the petal geometry
///
/// # Example
///
/// ```
/// use floraison_components::petal::{PetalParams, generate};
/// use floraison_components::Vec3;
///
/// let params = PetalParams {
///     length: 3.0,
///     width: 1.5,
///     tip_sharpness: 0.4,
///     base_width: 0.5,
///     curl: 0.3,
///     twist: 15.0,
///     lateral_curve: 0.0,
///     ruffle_freq: 2.0,
///     ruffle_amp: 0.1,
///     resolution: 16,
///     color: Vec3::ONE,
/// };
///
/// let petal = generate(&params);
/// assert!(petal.triangle_count() > 0);
/// ```
pub fn generate(params: &PetalParams) -> Mesh {
    // 1. Generate control grid
    let mut control_points = generate_control_grid(params);

    // 2. Apply deformations
    if params.curl.abs() > 0.001 {
        apply_curl(&mut control_points, params.curl);
    }
    if params.twist.abs() > 0.001 {
        apply_twist(&mut control_points, params.twist);
    }
    if params.lateral_curve.abs() > 0.001 {
        apply_lateral_curve(&mut control_points, params.lateral_curve);
    }
    if params.ruffle_freq.abs() > 0.001 && params.ruffle_amp.abs() > 0.001 {
        apply_ruffle(&mut control_points, params.ruffle_freq, params.ruffle_amp);
    }

    // 3. Create B-spline surface
    const ROWS: usize = 9;
    const COLS: usize = 5;

    // Transpose control grid: BSplineSurface expects control_points[u_index][v_index]
    // but our grid is [row][col] = [v_index][u_index]
    let mut transposed = vec![vec![Vec3::ZERO; ROWS]; COLS];
    for (row, row_data) in control_points.iter().enumerate().take(ROWS) {
        for (col, col_data) in transposed.iter_mut().enumerate().take(COLS) {
            col_data[row] = row_data[col];
        }
    }

    let surface = BSplineSurface {
        control_points: transposed,
        degree_u: 3,                                  // Cubic in u direction (width)
        degree_v: 3,                                  // Cubic in v direction (length)
        knots_u: generate_knot_vector(COLS, 3, true), // 5 control points in u
        knots_v: generate_knot_vector(ROWS, 3, true), // 9 control points in v
    };

    // 4. Tessellate surface
    let res = params.resolution;
    let mut mesh = Mesh::with_capacity((res + 1) * (res + 1), res * res * 2 * 3);

    // Generate front face vertices
    for i in 0..=res {
        let u = i as f32 / res as f32;
        for j in 0..=res {
            let v = j as f32 / res as f32;

            let pos = surface.evaluate(u, v);
            let normal = surface.normal(u, v);
            let uv_coord = Vec2::new(u, v);

            mesh.add_vertex(pos, normal, uv_coord, params.color);
        }
    }

    // Generate front face triangles
    for i in 0..res {
        for j in 0..res {
            let i0 = i * (res + 1) + j;
            let i1 = i0 + 1;
            let i2 = i0 + res + 1;
            let i3 = i2 + 1;

            mesh.add_triangle(i0 as u32, i2 as u32, i1 as u32);
            mesh.add_triangle(i1 as u32, i2 as u32, i3 as u32);
        }
    }

    // 5. Add back faces (flip normals and winding order)
    let front_vertex_count = mesh.vertex_count();

    // Duplicate vertices with flipped normals
    for i in 0..front_vertex_count {
        let pos = mesh.positions[i];
        let normal = -mesh.normals[i]; // Flip normal
        let uv = mesh.uvs[i];
        mesh.add_vertex(pos, normal, uv, params.color);
    }

    // Add back face triangles (reversed winding)
    for i in 0..res {
        for j in 0..res {
            let i0 = i * (res + 1) + j + front_vertex_count;
            let i1 = i0 + 1;
            let i2 = i0 + res + 1;
            let i3 = i2 + 1;

            // Reversed winding order
            mesh.add_triangle(i0 as u32, i1 as u32, i2 as u32);
            mesh.add_triangle(i1 as u32, i3 as u32, i2 as u32);
        }
    }

    mesh
}

/// Generate a petal mesh using legacy Bézier curve outline (deprecated)
///
/// This is the old flat petal generator. Use `generate()` instead for B-spline petals.
#[deprecated(note = "Use generate() with B-spline surfaces instead")]
#[allow(dead_code)]
fn generate_bezier_legacy(params: &PetalParams) -> Mesh {
    // Define key points for the petal outline
    let base_left = Vec2::new(-params.base_width / 2.0, 0.0);
    let base_right = Vec2::new(params.base_width / 2.0, 0.0);
    let tip = Vec2::new(0.0, params.length);

    // Point of maximum width (typically around 60% of length)
    let max_width_height = params.length * 0.6;
    let max_left = Vec2::new(-params.width / 2.0, max_width_height);
    let max_right = Vec2::new(params.width / 2.0, max_width_height);

    // Control point for tip sharpness
    // The closer this is to the tip, the sharper the point
    let tip_control_height = params.length * (1.0 - params.tip_sharpness * 0.5);
    let tip_control_width = params.width * (0.5 - params.tip_sharpness * 0.4);

    // Left side: base -> max width -> tip
    // Using cubic Bézier: base_left -> control1 -> control2 -> max_left
    let left_lower = sample_cubic_2d(
        base_left,
        Vec2::new(-params.base_width / 2.0, params.length * 0.2),
        Vec2::new(-params.width * 0.4, max_width_height * 0.6),
        max_left,
        params.resolution / 2,
    );

    // Max width to tip: max_left -> control -> tip
    let left_upper = sample_cubic_2d(
        max_left,
        Vec2::new(-params.width * 0.45, max_width_height * 1.2),
        Vec2::new(-tip_control_width, tip_control_height),
        tip,
        params.resolution / 2,
    );

    // Right side: tip -> max width -> base
    let right_upper = sample_cubic_2d(
        tip,
        Vec2::new(tip_control_width, tip_control_height),
        Vec2::new(params.width * 0.45, max_width_height * 1.2),
        max_right,
        params.resolution / 2,
    );

    let right_lower = sample_cubic_2d(
        max_right,
        Vec2::new(params.width * 0.4, max_width_height * 0.6),
        Vec2::new(params.base_width / 2.0, params.length * 0.2),
        base_right,
        params.resolution / 2,
    );

    // Combine all outline points
    let mut outline = Vec::new();
    outline.extend(left_lower);
    outline.extend(left_upper);
    outline.extend(right_upper);
    outline.extend(right_lower);

    // Remove duplicate points at connections
    outline.dedup_by(|a, b| (a.x - b.x).abs() < 0.0001 && (a.y - b.y).abs() < 0.0001);

    // Create mesh from outline using fan triangulation
    create_petal_mesh(&outline)
}

/// Create a mesh from a 2D outline using fan triangulation
fn create_petal_mesh(outline: &[Vec2]) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();

    // Calculate bounding box for UV mapping
    let min_x = outline.iter().map(|p| p.x).fold(f32::MAX, f32::min);
    let max_x = outline.iter().map(|p| p.x).fold(f32::MIN, f32::max);
    let min_y = outline.iter().map(|p| p.y).fold(f32::MAX, f32::min);
    let max_y = outline.iter().map(|p| p.y).fold(f32::MIN, f32::max);
    let width = max_x - min_x;
    let height = max_y - min_y;

    // Center point for fan triangulation
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;
    let center = Vec2::new(center_x, center_y);

    // Add center vertex
    positions.push(Vec3::new(center.x, center.y, 0.0));
    normals.push(Vec3::new(0.0, 0.0, 1.0)); // Normal pointing up (Z+)
    uvs.push(Vec2::new(
        (center.x - min_x) / width,
        (center.y - min_y) / height,
    ));
    colors.push(Vec3::ONE); // White default color

    // Add outline vertices
    for point in outline {
        positions.push(Vec3::new(point.x, point.y, 0.0));
        normals.push(Vec3::new(0.0, 0.0, 1.0));
        uvs.push(Vec2::new(
            (point.x - min_x) / width,
            (point.y - min_y) / height,
        ));
        colors.push(Vec3::ONE); // White default color
    }

    // Create fan triangles from center
    let outline_count = outline.len();
    for i in 0..outline_count {
        let next = (i + 1) % outline_count;
        indices.push(0); // Center
        indices.push((i + 1) as u32);
        indices.push((next + 1) as u32);
    }

    Mesh {
        positions,
        normals,
        uvs,
        colors,
        indices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_petal() {
        let mesh = generate(&PetalParams::default());

        assert!(mesh.vertex_count() > 0, "Should have vertices");
        assert!(mesh.triangle_count() > 0, "Should have triangles");

        // Check for valid geometry
        for pos in &mesh.positions {
            assert!(pos.is_finite(), "Position should be finite");
        }

        for normal in &mesh.normals {
            assert!(normal.is_finite(), "Normal should be finite");
            let len = normal.length();
            assert!(
                (len - 1.0).abs() < 0.01,
                "Normal should be normalized, got length {}",
                len
            );
        }
    }

    #[test]
    fn test_wide_petal() {
        let mesh = generate(&PetalParams::wide());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_narrow_petal() {
        let mesh = generate(&PetalParams::narrow());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_short_petal() {
        let mesh = generate(&PetalParams::short());

        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_petal_is_mostly_near_xy_plane() {
        // With no deformations, petal should be close to XY plane
        let params = PetalParams {
            curl: 0.0,
            twist: 0.0,
            lateral_curve: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            ..Default::default()
        };

        let mesh = generate(&params);

        // Most Z coordinates should be near 0 (close to XY plane)
        let avg_z: f32 =
            mesh.positions.iter().map(|p| p.z.abs()).sum::<f32>() / mesh.positions.len() as f32;
        assert!(
            avg_z < 0.5,
            "Petal with no deformations should be close to flat, avg |Z|={}",
            avg_z
        );
    }

    #[test]
    fn test_petal_dimensions() {
        let params = PetalParams {
            length: 5.0,
            width: 2.0,
            tip_sharpness: 0.5,
            base_width: 0.5,
            curl: 0.0,
            twist: 0.0,
            lateral_curve: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            resolution: 16,
            color: Vec3::ONE,
        };

        let mesh = generate(&params);

        // Check Y range (length)
        let min_y = mesh.positions.iter().map(|p| p.y).fold(f32::MAX, f32::min);
        let max_y = mesh.positions.iter().map(|p| p.y).fold(f32::MIN, f32::max);

        assert!(min_y < 0.5, "Base should be near 0, got min_y={}", min_y);
        assert!(
            (max_y - params.length).abs() < 0.1,
            "Tip should be at length {}, got max_y={}",
            params.length,
            max_y
        );

        // Check X range (width) - B-spline surface may not exactly match control grid bounds
        let min_x = mesh.positions.iter().map(|p| p.x).fold(f32::MAX, f32::min);
        let max_x = mesh.positions.iter().map(|p| p.x).fold(f32::MIN, f32::max);
        let actual_width = max_x - min_x;

        assert!(
            actual_width > 0.5 && actual_width < 3.0,
            "Width should be reasonable for petal: got {}",
            actual_width
        );
    }

    #[test]
    fn test_indices_in_bounds() {
        let mesh = generate(&PetalParams::default());
        let vertex_count = mesh.vertex_count() as u32;

        for &index in &mesh.indices {
            assert!(
                index < vertex_count,
                "Index {} out of bounds (vertex count: {})",
                index,
                vertex_count
            );
        }
    }

    #[test]
    fn test_normals_are_valid() {
        let mesh = generate(&PetalParams::default());

        // Double-sided mesh: half normals point up (+Z), half point down (-Z)
        // All normals should be normalized
        for normal in &mesh.normals {
            let length = normal.length();
            assert!(
                (length - 1.0).abs() < 0.1,
                "Normal should be normalized, got length {}",
                length
            );

            // Normal should be mostly perpendicular to XY plane (large |Z| component) for flat petal
            assert!(
                normal.z.abs() > 0.5,
                "Normal Z component should be significant for mostly-flat petal, got Z={}",
                normal.z
            );
        }
    }

    #[test]
    fn test_uv_mapping() {
        let mesh = generate(&PetalParams::default());

        // All UVs should be in [0, 1] range
        for uv in &mesh.uvs {
            assert!(
                uv.x >= 0.0 && uv.x <= 1.0,
                "UV x coordinate should be in [0,1]: {}",
                uv.x
            );
            assert!(
                uv.y >= 0.0 && uv.y <= 1.0,
                "UV y coordinate should be in [0,1]: {}",
                uv.y
            );
        }
    }

    #[test]
    fn test_tip_sharpness() {
        let sharp_params = PetalParams {
            length: 3.0,
            width: 1.5,
            tip_sharpness: 0.9,
            base_width: 0.5,
            curl: 0.0,
            twist: 0.0,
            lateral_curve: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            resolution: 16,
            color: Vec3::ONE,
        };

        let rounded_params = PetalParams {
            length: 3.0,
            width: 1.5,
            tip_sharpness: 0.1,
            base_width: 0.5,
            curl: 0.0,
            twist: 0.0,
            lateral_curve: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            resolution: 16,
            color: Vec3::ONE,
        };

        let sharp_mesh = generate(&sharp_params);
        let rounded_mesh = generate(&rounded_params);

        // Both should generate valid meshes
        assert!(sharp_mesh.vertex_count() > 0);
        assert!(rounded_mesh.vertex_count() > 0);

        // Verify all geometry is valid for both meshes
        for pos in &sharp_mesh.positions {
            assert!(pos.is_finite());
        }
        for pos in &rounded_mesh.positions {
            assert!(pos.is_finite());
        }
    }

    // Tests for B-spline control grid generation

    #[test]
    fn test_control_grid_dimensions() {
        let params = PetalParams::default();
        let grid = generate_control_grid(&params);

        assert_eq!(grid.len(), 9, "Should have 9 rows");
        assert_eq!(grid[0].len(), 5, "Should have 5 columns");

        // All rows should have same number of columns
        for row in &grid {
            assert_eq!(row.len(), 5);
        }
    }

    #[test]
    fn test_control_grid_is_flat() {
        let params = PetalParams::default();
        let grid = generate_control_grid(&params);

        // All Z coordinates should be 0 (flat in XY plane)
        for row in &grid {
            for point in row {
                assert!(
                    point.z.abs() < 0.0001,
                    "Control grid should be flat (Z=0), got Z={}",
                    point.z
                );
            }
        }
    }

    #[test]
    fn test_control_grid_base_and_tip() {
        let params = PetalParams {
            length: 5.0,
            width: 2.0,
            tip_sharpness: 0.5,
            base_width: 0.5,
            curl: 0.0,
            twist: 0.0,
            lateral_curve: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            resolution: 16,
            color: Vec3::ONE,
        };

        let grid = generate_control_grid(&params);

        // Base row (row 0) should be at y=0
        for point in &grid[0] {
            assert!(
                point.y.abs() < 0.01,
                "Base should be at y=0, got y={}",
                point.y
            );
        }

        // Tip row (row 8) should be at y=length
        for point in &grid[8] {
            assert!(
                (point.y - params.length).abs() < 0.01,
                "Tip should be at y={}, got y={}",
                params.length,
                point.y
            );
        }
    }

    #[test]
    fn test_control_grid_width_variation() {
        let params = PetalParams {
            length: 3.0,
            width: 2.0,
            tip_sharpness: 0.7,
            base_width: 0.4,
            curl: 0.0,
            twist: 0.0,
            lateral_curve: 0.0,
            ruffle_freq: 0.0,
            ruffle_amp: 0.0,
            resolution: 16,
            color: Vec3::ONE,
        };

        let grid = generate_control_grid(&params);

        // Base should be narrower than middle
        let base_width = grid[0][4].x - grid[0][0].x;
        let middle_width = grid[4][4].x - grid[4][0].x;

        assert!(
            middle_width > base_width,
            "Middle should be wider than base: middle={}, base={}",
            middle_width,
            base_width
        );

        // Tip should be narrower than middle
        let tip_width = grid[8][4].x - grid[8][0].x;

        assert!(
            tip_width < middle_width,
            "Tip should be narrower than middle: tip={}, middle={}",
            tip_width,
            middle_width
        );
    }

    #[test]
    fn test_control_grid_symmetric() {
        let params = PetalParams::default();
        let grid = generate_control_grid(&params);

        // Each row should be symmetric around x=0
        for row in &grid {
            let left_edge = row[0].x;
            let right_edge = row[4].x;

            assert!(
                (left_edge + right_edge).abs() < 0.01,
                "Row should be symmetric around x=0: left={}, right={}",
                left_edge,
                right_edge
            );

            // Center column should be at x=0
            assert!(
                row[2].x.abs() < 0.01,
                "Center should be at x=0, got x={}",
                row[2].x
            );
        }
    }

    #[test]
    fn test_control_grid_all_finite() {
        let params = PetalParams::default();
        let grid = generate_control_grid(&params);

        for row in &grid {
            for point in row {
                assert!(point.is_finite(), "All control points should be finite");
            }
        }
    }

    // Tests for deformation functions

    #[test]
    fn test_apply_curl_upward() {
        let params = PetalParams::default();
        let mut grid = generate_control_grid(&params);

        // Apply upward curl
        apply_curl(&mut grid, 0.5);

        // Base should remain near Z=0
        for point in &grid[0] {
            assert!(
                point.z.abs() < 0.1,
                "Base should not curl much: Z={}",
                point.z
            );
        }

        // Tip should curl upward (positive Z)
        for point in &grid[8] {
            assert!(point.z > 0.0, "Tip should curl upward: Z={}", point.z);
        }

        // All points should remain finite
        for row in &grid {
            for point in row {
                assert!(point.is_finite());
            }
        }
    }

    #[test]
    fn test_apply_curl_downward() {
        let params = PetalParams::default();
        let mut grid = generate_control_grid(&params);

        // Apply downward curl
        apply_curl(&mut grid, -0.5);

        // Tip should curl downward (negative Z)
        for point in &grid[8] {
            assert!(point.z < 0.0, "Tip should curl downward: Z={}", point.z);
        }
    }

    #[test]
    fn test_apply_twist() {
        let params = PetalParams::default();
        let mut grid = generate_control_grid(&params);

        // Record base positions
        let base_left_x = grid[0][0].x;
        let base_right_x = grid[0][4].x;

        // Apply 45-degree twist
        apply_twist(&mut grid, 45.0);

        // Base should not twist much
        assert!(
            (grid[0][0].x - base_left_x).abs() < 0.1,
            "Base should not twist"
        );
        assert!(
            (grid[0][4].x - base_right_x).abs() < 0.1,
            "Base should not twist"
        );

        // Tip should be twisted (some Z displacement)
        let tip_has_z = grid[8].iter().any(|p| p.z.abs() > 0.1);
        assert!(tip_has_z, "Tip should have Z displacement from twist");

        // All points should remain finite
        for row in &grid {
            for point in row {
                assert!(point.is_finite());
            }
        }
    }

    #[test]
    fn test_apply_ruffle() {
        let params = PetalParams::default();
        let mut grid = generate_control_grid(&params);

        // Apply ruffle
        apply_ruffle(&mut grid, 3.0, 0.2);

        // Center column should have minimal Z displacement (not at edges)
        for row in &grid {
            assert!(
                row[2].z.abs() < 0.1,
                "Center should not ruffle much: Z={}",
                row[2].z
            );
        }

        // Edge columns should have some Z displacement
        let edges_have_z = grid
            .iter()
            .any(|row| row[0].z.abs() > 0.05 || row[4].z.abs() > 0.05);
        assert!(edges_have_z, "Edges should have Z displacement from ruffle");

        // All points should remain finite
        for row in &grid {
            for point in row {
                assert!(point.is_finite());
            }
        }
    }

    #[test]
    fn test_combined_deformations() {
        let params = PetalParams::default();
        let mut grid = generate_control_grid(&params);

        // Apply all three deformations
        apply_curl(&mut grid, 0.3);
        apply_twist(&mut grid, 30.0);
        apply_ruffle(&mut grid, 2.0, 0.15);

        // All points should remain finite
        for row in &grid {
            for point in row {
                assert!(
                    point.is_finite(),
                    "Combined deformations should produce finite values"
                );
            }
        }

        // Grid should still have expected structure
        assert_eq!(grid.len(), 9);
        assert_eq!(grid[0].len(), 5);
    }

    #[test]
    fn test_zero_deformations() {
        let params = PetalParams::default();
        let mut grid = generate_control_grid(&params);
        let original_grid = grid.clone();

        // Apply zero deformations
        apply_curl(&mut grid, 0.0);
        apply_twist(&mut grid, 0.0);
        apply_ruffle(&mut grid, 0.0, 0.0);

        // Grid should be unchanged (within floating point tolerance)
        for (i, row) in grid.iter().enumerate() {
            for (j, point) in row.iter().enumerate() {
                let orig = original_grid[i][j];
                assert!(
                    (point.x - orig.x).abs() < 0.001
                        && (point.y - orig.y).abs() < 0.001
                        && (point.z - orig.z).abs() < 0.001,
                    "Zero deformations should not change grid"
                );
            }
        }
    }
}
