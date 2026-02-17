export interface Style {
  id: string;
  label: string;
  icon: string;
  promptFragment: string;
}

export interface ColorScheme {
  id: string;
  label: string;
  icon: string;
  description: string;
}

export const STYLES: Style[] = [
  { id: "geometric", label: "Geometric", icon: "\u25B3", promptFragment: "geometric shapes, triangles, hexagons, tessellations, mathematical patterns" },
  { id: "gradient", label: "Gradient", icon: "\u25C9", promptFragment: "smooth color gradients, color transitions, gradient mesh, blending colors" },
  { id: "minimal", label: "Minimal", icon: "\u2014", promptFragment: "minimalist composition, few elements, negative space, simple and clean" },
  { id: "lineart", label: "Line Art", icon: "\u2571", promptFragment: "continuous line art, thin strokes, flowing lines, wireframe aesthetic" },
  { id: "isometric", label: "Isometric", icon: "\u2B21", promptFragment: "isometric perspective, 3D blocks, isometric shapes, low-poly style" },
  { id: "abstract", label: "Abstract", icon: "\u25CD", promptFragment: "abstract vector shapes, organic curves, bold composition, overlapping forms" },
  { id: "sunburst", label: "Sunburst", icon: "\u2600", promptFragment: "radiating rays, concentric arcs, radial burst, sun rays pattern" },
  { id: "waves", label: "Waves", icon: "\u223F", promptFragment: "wave patterns, flowing curves, undulating lines, rhythmic patterns" },
  { id: "dots", label: "Dots & Circles", icon: "\u25CF", promptFragment: "dot patterns, concentric circles, polka dots, radial patterns, halftone" },
  { id: "silkflow", label: "Silk Flow", icon: "\u2740", promptFragment: "flowing silk petals, smooth curves radiating from center, translucent layers" },
];

export const COLOR_SCHEMES: ColorScheme[] = [
  { id: "retro", label: "Retro", icon: "\u25C6", description: "Warm vintage, earth tones, 70s palette" },
  { id: "neon", label: "Neon", icon: "\u26A1", description: "Electric brights on dark backgrounds" },
  { id: "cyberpunk", label: "Cyberpunk", icon: "\u2B23", description: "Hot pink, electric cyan, chrome" },
  { id: "synthwave", label: "Synthwave", icon: "\u266B", description: "Purple-to-pink gradients, 80s sunset" },
];

// Backward compat
export type Category = Style;
export const CATEGORIES = STYLES;
