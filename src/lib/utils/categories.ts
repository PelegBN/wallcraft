export interface Category {
  id: string;
  label: string;
  icon: string;
  promptFragment: string;
}

export const CATEGORIES: Category[] = [
  { id: "nature", label: "Nature", icon: "🌿", promptFragment: "lush nature landscape, forests, rivers" },
  { id: "abstract", label: "Abstract", icon: "🎨", promptFragment: "abstract art, geometric shapes, flowing colors" },
  { id: "space", label: "Space", icon: "🌌", promptFragment: "outer space, galaxies, nebulae, stars" },
  { id: "cityscape", label: "Cityscape", icon: "🏙️", promptFragment: "urban skyline, city lights, architecture" },
  { id: "fantasy", label: "Fantasy", icon: "🐉", promptFragment: "fantasy landscape, magical, mythical" },
  { id: "minimalist", label: "Minimalist", icon: "⬜", promptFragment: "minimalist design, clean lines, simple colors" },
  { id: "ocean", label: "Ocean", icon: "🌊", promptFragment: "ocean waves, underwater, coral reef, sea" },
  { id: "mountains", label: "Mountains", icon: "🏔️", promptFragment: "mountain peaks, alpine landscape, snow-capped" },
  { id: "cyberpunk", label: "Cyberpunk", icon: "🤖", promptFragment: "cyberpunk city, neon lights, futuristic" },
  { id: "seasons", label: "Seasons", icon: "🍂", promptFragment: "seasonal landscape, autumn leaves, spring bloom" },
];
