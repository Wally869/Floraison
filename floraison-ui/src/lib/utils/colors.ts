/**
 * Color Utility Functions
 *
 * Conversions between color spaces and color manipulation utilities.
 */

/**
 * Convert HSL color to RGB
 *
 * @param h - Hue (0-1, where 0=red, 0.33=green, 0.67=blue)
 * @param s - Saturation (0-1, where 0=gray, 1=full color)
 * @param l - Lightness (0-1, where 0=black, 0.5=normal, 1=white)
 * @returns [r, g, b] tuple with values in 0-1 range
 *
 * @example
 * ```typescript
 * hslToRgb(0.0, 1.0, 0.5)  // Pure red: [1, 0, 0]
 * hslToRgb(0.33, 1.0, 0.5) // Pure green: [0, 1, 0]
 * hslToRgb(0.5, 0.5, 0.75) // Pastel cyan
 * ```
 */
export function hslToRgb(h: number, s: number, l: number): [number, number, number] {
	let r, g, b;

	if (s === 0) {
		// Achromatic (gray)
		r = g = b = l;
	} else {
		const hue2rgb = (p: number, q: number, t: number): number => {
			if (t < 0) t += 1;
			if (t > 1) t -= 1;
			if (t < 1 / 6) return p + (q - p) * 6 * t;
			if (t < 1 / 2) return q;
			if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
			return p;
		};

		const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
		const p = 2 * l - q;

		r = hue2rgb(p, q, h + 1 / 3);
		g = hue2rgb(p, q, h);
		b = hue2rgb(p, q, h - 1 / 3);
	}

	return [r, g, b];
}

/**
 * Convert RGB color to hex string
 *
 * @param rgb - RGB tuple with values in 0-1 range
 * @returns Hex color string (e.g., "#ff5733")
 *
 * @example
 * ```typescript
 * rgbToHex([1.0, 0.0, 0.0])     // "#ff0000" (red)
 * rgbToHex([0.5, 0.75, 1.0])    // "#80bfff" (light blue)
 * ```
 */
export function rgbToHex(rgb: [number, number, number]): string {
	const [r, g, b] = rgb;
	const toHex = (n: number) => {
		const hex = Math.round(n * 255)
			.toString(16)
			.padStart(2, '0');
		return hex;
	};
	return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

/**
 * Convert hex color string to RGB
 *
 * @param hex - Hex color string (with or without #)
 * @returns [r, g, b] tuple with values in 0-1 range
 *
 * @example
 * ```typescript
 * hexToRgb("#ff0000")  // [1.0, 0.0, 0.0] (red)
 * hexToRgb("80bfff")   // [0.5, 0.75, 1.0] (light blue)
 * ```
 */
export function hexToRgb(hex: string): [number, number, number] {
	const clean = hex.replace('#', '');
	const r = parseInt(clean.substring(0, 2), 16) / 255;
	const g = parseInt(clean.substring(2, 4), 16) / 255;
	const b = parseInt(clean.substring(4, 6), 16) / 255;
	return [r, g, b];
}

/**
 * Interpolate between two RGB colors
 *
 * @param color1 - First color
 * @param color2 - Second color
 * @param t - Interpolation factor (0-1, where 0=color1, 1=color2)
 * @returns Interpolated color
 *
 * @example
 * ```typescript
 * const red = [1.0, 0.0, 0.0] as [number, number, number];
 * const blue = [0.0, 0.0, 1.0] as [number, number, number];
 * lerpColor(red, blue, 0.5)  // Purple: [0.5, 0.0, 0.5]
 * ```
 */
export function lerpColor(
	color1: [number, number, number],
	color2: [number, number, number],
	t: number
): [number, number, number] {
	return [
		color1[0] + (color2[0] - color1[0]) * t,
		color1[1] + (color2[1] - color1[1]) * t,
		color1[2] + (color2[2] - color1[2]) * t
	];
}
