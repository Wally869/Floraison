/**
 * Curve Generation Utilities
 *
 * Functions for generating 3D curve control points from simple bend parameters.
 * These curves are used to create curved pistil styles and stamen filaments.
 */

/**
 * Generate Catmull-Rom curve control points for a bent stem
 *
 * Creates a smooth curve that arcs both sideways (X) and vertically (Y) based on bend/droop amounts.
 * The curve starts at origin (0,0,0) and ends at full length with a graceful arc.
 *
 * Technical note: Catmull-Rom splines only pass through interior control points.
 * We use extrapolated control points (P0 and P4) to define tangents while ensuring
 * the curve passes through the actual start (P1), middle (P2), and end (P3) points.
 *
 * @param length - Total length of the stem
 * @param bendAmount - Horizontal bend intensity (0=straight, 1=dramatic arc, max 50% of length)
 * @param droopAmount - Vertical droop (-1=droop down, 0=straight, 1=lift up)
 * @param direction - Direction of horizontal bend: 1 = +X, -1 = -X
 * @returns Array of 5 Vec3 control points [x, y, z], or null if no bend/droop
 *
 * @example
 * ```typescript
 * // Generate a stamen that curves right and droops down
 * const curve = generateBendCurve(2.0, 0.3, -0.5, 1);
 * // Returns: [p0_control, start, middle, end, p4_control]
 * ```
 */
export function generateBendCurve(
	length: number,
	bendAmount: number,
	droopAmount: number = 0.0,
	direction: number = 1
): Array<[number, number, number]> | null {
	// No curve if both bend and droop are effectively zero
	if (bendAmount < 0.01 && Math.abs(droopAmount) < 0.01) return null;

	// Maximum sideways displacement as fraction of length
	const maxDisplacement = length * 0.5 * bendAmount;
	const dir = direction;

	// Vertical droop: negative values decrease Y (droop down), positive increases Y (lift up)
	// Scale droop by length for proportional effect
	const droopScale = length * 0.4;

	// Define the actual curve points we want to pass through
	const start: [number, number, number] = [0, 0, 0]; // Base at origin
	const middle: [number, number, number] = [
		maxDisplacement * 0.7 * dir,                    // X: peak sideways bend
		length * 0.5 - droopAmount * droopScale * 0.5,  // Y: droop=-1 makes middle HIGHER (arch)
		0
	];
	const end: [number, number, number] = [
		maxDisplacement * 0.4 * dir,               // X: curve back slightly
		length,                                    // Y: always end at target length
		0
	];

	// Catmull-Rom passes through interior points only (P1 to P2 for a 4-point curve)
	// So we need: [P0, P1, P2, P3, P4] where curve goes P1 -> P2 -> P3
	// P0 and P4 are control points that define tangents

	// Extrapolate P0 (control before start) - projects backward from start using start->middle direction
	const p0: [number, number, number] = [
		start[0] - (middle[0] - start[0]) * 0.5,
		start[1] - (middle[1] - start[1]) * 0.5,
		start[2] - (middle[2] - start[2]) * 0.5
	];

	// Extrapolate P4 (control after end) - projects forward from end using middle->end direction
	const p4: [number, number, number] = [
		end[0] + (end[0] - middle[0]) * 0.5,
		end[1] + (end[1] - middle[1]) * 0.5,
		end[2] + (end[2] - middle[2]) * 0.5
	];

	// Return 5-point Catmull-Rom curve: [control, start, middle, end, control]
	const points: Array<[number, number, number]> = [
		p0,     // P0 - extrapolated control point before start
		start,  // P1 - actual start (curve begins here)
		middle, // P2 - middle bend point (curve passes through)
		end,    // P3 - actual end (curve ends here)
		p4      // P4 - extrapolated control point after end
	];

	return points;
}

/**
 * Generate random bend direction (-1 or 1)
 *
 * Used for adding variety to random flower generation.
 *
 * @returns -1 (bend left) or 1 (bend right)
 *
 * @example
 * ```typescript
 * const direction = randomBendDirection();
 * const curve = generateBendCurve(2.0, 0.5, direction);
 * ```
 */
export function randomBendDirection(): number {
	return Math.random() < 0.5 ? -1 : 1;
}
