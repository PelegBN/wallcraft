use std::f64::consts::PI;
use std::path::PathBuf;

use rand::Rng;

use crate::error::AppError;

/// Color palette: a background color and a set of accent colors.
struct Palette {
    bg: &'static str,
    colors: &'static [&'static str],
}

// --- Named color scheme palettes ---

const DEFAULT_PALETTES: &[Palette] = &[
    Palette { bg: "#0f172a", colors: &["#3b82f6", "#8b5cf6", "#06b6d4", "#6366f1", "#2dd4bf"] },
    Palette { bg: "#1a1a2e", colors: &["#e94560", "#533483", "#0f3460", "#16213e", "#e94560"] },
    Palette { bg: "#0d1117", colors: &["#58a6ff", "#bc8cff", "#7ee787", "#ffa657", "#ff7b72"] },
    Palette { bg: "#1e1b4b", colors: &["#818cf8", "#c084fc", "#f472b6", "#fb923c", "#a78bfa"] },
    Palette { bg: "#022c22", colors: &["#34d399", "#6ee7b7", "#a7f3d0", "#059669", "#10b981"] },
    Palette { bg: "#2d1b69", colors: &["#f97316", "#eab308", "#ef4444", "#ec4899", "#8b5cf6"] },
    Palette { bg: "#18181b", colors: &["#f4f4f5", "#a1a1aa", "#71717a", "#52525b", "#d4d4d8"] },
    Palette { bg: "#1c1917", colors: &["#f59e0b", "#d97706", "#b45309", "#fbbf24", "#fde68a"] },
];

const RETRO_PALETTES: &[Palette] = &[
    Palette { bg: "#2b1a0e", colors: &["#D94F30", "#E8832A", "#F2B824", "#F5E6C8", "#2A9D8F", "#1D6B8A"] },
    Palette { bg: "#1a1208", colors: &["#C1440E", "#E77728", "#F4A742", "#F7DC6F", "#8E6C4A", "#5B3A29"] },
    Palette { bg: "#0f2421", colors: &["#2A9D8F", "#264653", "#E9C46A", "#8AB17D", "#287271", "#5B8E7D"] },
    Palette { bg: "#0e1b2e", colors: &["#457B9D", "#1D3557", "#A8DADC", "#9DB4C0", "#6B8F71", "#E0AFA0"] },
    Palette { bg: "#1f1a1e", colors: &["#6D6875", "#B5838D", "#E5989B", "#FFB4A2", "#FFCDB2", "#9E8576"] },
    Palette { bg: "#1e1a14", colors: &["#CB997E", "#DDBEA9", "#B7B7A4", "#A5A58D", "#6B705C", "#FFE8D6"] },
    Palette { bg: "#1a1f0e", colors: &["#606C38", "#283618", "#FEFAE0", "#DDA15E", "#BC6C25", "#9B8E7E"] },
    Palette { bg: "#0e0d26", colors: &["#48BFE3", "#5390D9", "#6930C3", "#7400B8", "#64DFDF", "#80FFDB"] },
    Palette { bg: "#1e152a", colors: &["#FFD6FF", "#E7C6FF", "#C8B6FF", "#B8C0FF", "#BBD0FF", "#FFFFFC"] },
];

const NEON_PALETTES: &[Palette] = &[
    Palette { bg: "#0a0a0a", colors: &["#ff0055", "#00ff87", "#00d4ff", "#ff00ff", "#ffff00"] },
    Palette { bg: "#050510", colors: &["#ff3366", "#33ffcc", "#3366ff", "#ff66ff", "#66ff33"] },
    Palette { bg: "#0d0d0d", colors: &["#ff2d00", "#00ffc8", "#7b2dff", "#ff6ec7", "#00ff00"] },
    Palette { bg: "#080012", colors: &["#fe019a", "#04d9ff", "#39ff14", "#ff6700", "#bc13fe"] },
];

const CYBERPUNK_PALETTES: &[Palette] = &[
    Palette { bg: "#0a0014", colors: &["#ff2a6d", "#05d9e8", "#d1f7ff", "#ff6c11", "#01012b"] },
    Palette { bg: "#0d0221", colors: &["#ff2975", "#00fff1", "#ff00a0", "#ffd319", "#0abdc6"] },
    Palette { bg: "#120024", colors: &["#f72585", "#4cc9f0", "#7209b7", "#fca311", "#e5e5e5"] },
    Palette { bg: "#0b0c10", colors: &["#c5c6c7", "#45a29e", "#66fcf1", "#ff003c", "#1f2833"] },
];

const SYNTHWAVE_PALETTES: &[Palette] = &[
    Palette { bg: "#0d0028", colors: &["#ff71ce", "#01cdfe", "#b967ff", "#fffb96", "#05ffa1"] },
    Palette { bg: "#1a0033", colors: &["#f72585", "#b5179e", "#7209b7", "#560bad", "#480ca8"] },
    Palette { bg: "#0f0326", colors: &["#ff6ad5", "#c774e8", "#ad8cff", "#8795e8", "#94d0ff"] },
    Palette { bg: "#170040", colors: &["#ff2281", "#ff6e27", "#fff338", "#88e8f2", "#7b68ee"] },
];

/// Resolve a palette from selected color scheme IDs.
/// No schemes → random default. One scheme → random from that pool. Multiple → blend.
fn resolve_palette(rng: &mut impl Rng, color_schemes: &[String]) -> Palette {
    if color_schemes.is_empty() {
        let p = &DEFAULT_PALETTES[rng.gen_range(0..DEFAULT_PALETTES.len())];
        return Palette { bg: p.bg, colors: p.colors };
    }

    let mut pools: Vec<&[Palette]> = Vec::new();
    for scheme in color_schemes {
        match scheme.as_str() {
            "retro" => pools.push(RETRO_PALETTES),
            "neon" => pools.push(NEON_PALETTES),
            "cyberpunk" => pools.push(CYBERPUNK_PALETTES),
            "synthwave" => pools.push(SYNTHWAVE_PALETTES),
            _ => {}
        }
    }

    if pools.is_empty() {
        let p = &DEFAULT_PALETTES[rng.gen_range(0..DEFAULT_PALETTES.len())];
        return Palette { bg: p.bg, colors: p.colors };
    }

    if pools.len() == 1 {
        let pool = pools[0];
        let p = &pool[rng.gen_range(0..pool.len())];
        return Palette { bg: p.bg, colors: p.colors };
    }

    // Multiple schemes: pick one palette from each, merge their colors
    let mut all_colors: Vec<&'static str> = Vec::new();
    let mut bgs: Vec<&'static str> = Vec::new();
    for pool in &pools {
        let p = &pool[rng.gen_range(0..pool.len())];
        bgs.push(p.bg);
        all_colors.extend_from_slice(p.colors);
    }

    let bg = bgs[rng.gen_range(0..bgs.len())];
    // Leak the combined slice (trivial memory, runs once per generation)
    let colors: &'static [&'static str] = Box::leak(all_colors.into_boxed_slice());

    Palette { bg, colors }
}

/// Generate an SVG string and rasterize it to a PNG file at the given dimensions.
pub fn generate(
    styles: &[String],
    color_schemes: &[String],
    _custom_prompt: Option<&str>,
    target_width: u32,
    target_height: u32,
    output_path: &PathBuf,
) -> Result<(), AppError> {
    let mut rng = rand::thread_rng();
    let palette = resolve_palette(&mut rng, color_schemes);
    let w = target_width as f64;
    let h = target_height as f64;

    let mut shapes = String::new();

    let cats: Vec<&str> = if styles.is_empty() {
        vec!["geometric", "gradient"]
    } else {
        styles.iter().map(|s| s.as_str()).collect()
    };

    for cat in &cats {
        match *cat {
            "geometric" => shapes.push_str(&gen_geometric(&mut rng, &palette, w, h)),
            "gradient" => shapes.push_str(&gen_gradients(&mut rng, &palette, w, h)),
            "minimal" => shapes.push_str(&gen_minimal(&mut rng, &palette, w, h)),
            "lineart" => shapes.push_str(&gen_lineart(&mut rng, &palette, w, h)),
            "isometric" => shapes.push_str(&gen_isometric(&mut rng, &palette, w, h)),
            "abstract" => shapes.push_str(&gen_abstract(&mut rng, &palette, w, h)),
            "waves" => shapes.push_str(&gen_waves(&mut rng, &palette, w, h)),
            "dots" => shapes.push_str(&gen_dots(&mut rng, &palette, w, h)),
            "silkflow" => shapes.push_str(&gen_silkflow(&mut rng, &palette, w, h)),
            "sunburst" => shapes.push_str(&gen_sunburst(&mut rng, &palette, w, h)),
            _ => shapes.push_str(&gen_geometric(&mut rng, &palette, w, h)),
        }
    }

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">
  <rect width="{w}" height="{h}" fill="{bg}"/>
  {shapes}
</svg>"#,
        w = w,
        h = h,
        bg = palette.bg,
        shapes = shapes,
    );

    rasterize_svg(&svg, target_width, target_height, output_path)
}

fn rasterize_svg(
    svg_str: &str,
    width: u32,
    height: u32,
    output_path: &PathBuf,
) -> Result<(), AppError> {
    let tree = resvg::usvg::Tree::from_str(svg_str, &resvg::usvg::Options::default())
        .map_err(|e| AppError::Generation(format!("Failed to parse SVG: {}", e)))?;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| AppError::Generation("Failed to create pixel buffer".into()))?;

    let tree_size = tree.size();
    let sx = width as f32 / tree_size.width();
    let sy = height as f32 / tree_size.height();
    let transform = resvg::tiny_skia::Transform::from_scale(sx, sy);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    pixmap
        .save_png(output_path)
        .map_err(|e| AppError::Generation(format!("Failed to save PNG: {}", e)))?;

    Ok(())
}

// --- Pattern generators ---

fn pick(rng: &mut impl Rng, palette: &Palette) -> &'static str {
    palette.colors[rng.gen_range(0..palette.colors.len())]
}

fn gen_geometric(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let mut s = String::new();
    let count = rng.gen_range(15..30);

    for _ in 0..count {
        let color = pick(rng, palette);
        let opacity = rng.gen_range(0.15..0.7);
        let shape_type = rng.gen_range(0..4);

        match shape_type {
            0 => {
                // Triangle
                let cx = rng.gen_range(0.0..w);
                let cy = rng.gen_range(0.0..h);
                let size = rng.gen_range(w * 0.05..w * 0.25);
                let angle: f64 = rng.gen_range(0.0..PI * 2.0);
                let points: Vec<String> = (0..3)
                    .map(|i| {
                        let a = angle + (i as f64) * PI * 2.0 / 3.0;
                        format!("{:.1},{:.1}", cx + a.cos() * size, cy + a.sin() * size)
                    })
                    .collect();
                s.push_str(&format!(
                    r#"  <polygon points="{}" fill="{}" opacity="{:.2}"/>"#,
                    points.join(" "), color, opacity
                ));
                s.push('\n');
            }
            1 => {
                // Hexagon
                let cx = rng.gen_range(0.0..w);
                let cy = rng.gen_range(0.0..h);
                let size = rng.gen_range(w * 0.03..w * 0.15);
                let points: Vec<String> = (0..6)
                    .map(|i| {
                        let a = (i as f64) * PI / 3.0;
                        format!("{:.1},{:.1}", cx + a.cos() * size, cy + a.sin() * size)
                    })
                    .collect();
                s.push_str(&format!(
                    r#"  <polygon points="{}" fill="{}" opacity="{:.2}"/>"#,
                    points.join(" "), color, opacity
                ));
                s.push('\n');
            }
            2 => {
                // Rectangle
                let x = rng.gen_range(-w * 0.1..w);
                let y = rng.gen_range(-h * 0.1..h);
                let rw = rng.gen_range(w * 0.05..w * 0.3);
                let rh = rng.gen_range(h * 0.05..h * 0.3);
                let angle = rng.gen_range(0.0..360.0f64);
                s.push_str(&format!(
                    r#"  <rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="{}" opacity="{:.2}" transform="rotate({:.1} {:.1} {:.1})"/>"#,
                    x, y, rw, rh, color, opacity, angle, x + rw / 2.0, y + rh / 2.0
                ));
                s.push('\n');
            }
            _ => {
                // Circle
                let cx = rng.gen_range(0.0..w);
                let cy = rng.gen_range(0.0..h);
                let r = rng.gen_range(w * 0.02..w * 0.15);
                s.push_str(&format!(
                    r#"  <circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="{}" opacity="{:.2}"/>"#,
                    cx, cy, r, color, opacity
                ));
                s.push('\n');
            }
        }
    }
    s
}

fn gen_gradients(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let mut defs = String::new();
    let mut shapes = String::new();

    let angle = rng.gen_range(15.0..165.0f64);
    let rad = angle.to_radians();
    let x1 = 50.0 - 50.0 * rad.cos();
    let y1 = 50.0 - 50.0 * rad.sin();
    let x2 = 50.0 + 50.0 * rad.cos();
    let y2 = 50.0 + 50.0 * rad.sin();

    defs.push_str(&format!(
        r#"    <linearGradient id="mainGrad" x1="{:.0}%" y1="{:.0}%" x2="{:.0}%" y2="{:.0}%">
"#,
        x1, y1, x2, y2
    ));
    let stop_count = palette.colors.len().min(5);
    for (i, &color) in palette.colors.iter().take(stop_count).enumerate() {
        let offset = (i as f64 / (stop_count - 1) as f64) * 100.0;
        defs.push_str(&format!(
            r#"      <stop offset="{:.0}%" stop-color="{}"/>
"#,
            offset, color
        ));
    }
    defs.push_str("    </linearGradient>\n");

    shapes.push_str(&format!(
        r#"  <rect width="{:.1}" height="{:.1}" fill="url(#mainGrad)" opacity="0.7"/>"#,
        w, h
    ));
    shapes.push('\n');

    let orb_count = rng.gen_range(2..5);
    for i in 0..orb_count {
        let id = format!("radGrad{}", i);
        let color = pick(rng, palette);
        let cx_pct = rng.gen_range(15.0..85.0);
        let cy_pct = rng.gen_range(15.0..85.0);

        defs.push_str(&format!(
            r#"    <radialGradient id="{}" cx="{:.0}%" cy="{:.0}%" r="50%">
      <stop offset="0%" stop-color="{}" stop-opacity="0.6"/>
      <stop offset="100%" stop-color="{}" stop-opacity="0"/>
    </radialGradient>
"#,
            id, cx_pct, cy_pct, color, color
        ));

        let rx = rng.gen_range(w * 0.25..w * 0.6);
        let ry = rng.gen_range(h * 0.25..h * 0.6);
        let cx = cx_pct / 100.0 * w;
        let cy = cy_pct / 100.0 * h;
        shapes.push_str(&format!(
            r#"  <ellipse cx="{:.1}" cy="{:.1}" rx="{:.1}" ry="{:.1}" fill="url(#{})"/>"#,
            cx, cy, rx, ry, id
        ));
        shapes.push('\n');
    }

    let band_count = rng.gen_range(2..4);
    let perp_angle = angle + 90.0;
    let perp_rad = perp_angle.to_radians();
    for i in 0..band_count {
        let t = (i as f64 + 1.0) / (band_count as f64 + 1.0);
        let bx = w * t;
        let by = h * t;
        let len = (w * w + h * h).sqrt() * 0.6;
        let bx1 = bx - perp_rad.cos() * len;
        let by1 = by - perp_rad.sin() * len;
        let bx2 = bx + perp_rad.cos() * len;
        let by2 = by + perp_rad.sin() * len;
        let color = pick(rng, palette);
        let sw = rng.gen_range(w * 0.02..w * 0.06);

        shapes.push_str(&format!(
            r#"  <line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="{:.1}" opacity="0.15"/>"#,
            bx1, by1, bx2, by2, color, sw
        ));
        shapes.push('\n');
    }

    format!("  <defs>\n{}</defs>\n{}", defs, shapes)
}

fn gen_minimal(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let mut s = String::new();
    let count = rng.gen_range(3..6);

    for _ in 0..count {
        let color = pick(rng, palette);
        let opacity = rng.gen_range(0.3..0.8);
        let shape = rng.gen_range(0..3);

        match shape {
            0 => {
                let cx = rng.gen_range(w * 0.2..w * 0.8);
                let cy = rng.gen_range(h * 0.2..h * 0.8);
                let r = rng.gen_range(w * 0.05..w * 0.2);
                s.push_str(&format!(
                    r#"  <circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="{}" opacity="{:.2}"/>"#,
                    cx, cy, r, color, opacity
                ));
                s.push('\n');
            }
            1 => {
                let y = rng.gen_range(h * 0.1..h * 0.9);
                let x1 = rng.gen_range(0.0..w * 0.3);
                let x2 = rng.gen_range(w * 0.7..w);
                s.push_str(&format!(
                    r#"  <line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width="2" opacity="{:.2}"/>"#,
                    x1, y, x2, y, color, opacity
                ));
                s.push('\n');
            }
            _ => {
                let size = rng.gen_range(w * 0.02..w * 0.08);
                let x = rng.gen_range(w * 0.1..w * 0.9 - size);
                let y = rng.gen_range(h * 0.1..h * 0.9 - size);
                s.push_str(&format!(
                    r#"  <rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="{}" opacity="{:.2}"/>"#,
                    x, y, size, size, color, opacity
                ));
                s.push('\n');
            }
        }
    }
    s
}

fn gen_lineart(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let mut s = String::new();
    let count = rng.gen_range(5..12);

    for _ in 0..count {
        let color = pick(rng, palette);
        let opacity = rng.gen_range(0.3..0.8);
        let stroke_w = rng.gen_range(1.0..4.0);
        let points = rng.gen_range(3..8);

        let mut path = format!("M {:.1} {:.1}", rng.gen_range(0.0..w), rng.gen_range(0.0..h));
        for _ in 1..points {
            let cp1x = rng.gen_range(0.0..w);
            let cp1y = rng.gen_range(0.0..h);
            let cp2x = rng.gen_range(0.0..w);
            let cp2y = rng.gen_range(0.0..h);
            let ex = rng.gen_range(0.0..w);
            let ey = rng.gen_range(0.0..h);
            path.push_str(&format!(
                " C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}",
                cp1x, cp1y, cp2x, cp2y, ex, ey
            ));
        }

        s.push_str(&format!(
            r#"  <path d="{}" fill="none" stroke="{}" stroke-width="{:.1}" opacity="{:.2}" stroke-linecap="round"/>"#,
            path, color, stroke_w, opacity
        ));
        s.push('\n');
    }
    s
}

fn gen_isometric(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let mut s = String::new();
    let cell = w / rng.gen_range(8.0..14.0);
    let cols = (w / cell) as i32 + 2;
    let rows = (h / (cell * 0.6)) as i32 + 2;

    for row in -1..rows {
        for col in -1..cols {
            if rng.gen_range(0.0..1.0) > 0.6 {
                continue;
            }
            let x = col as f64 * cell + if row % 2 == 0 { 0.0 } else { cell * 0.5 };
            let y = row as f64 * cell * 0.6;
            let height = rng.gen_range(cell * 0.3..cell * 1.2);
            let color = pick(rng, palette);
            let opacity = rng.gen_range(0.3..0.7);

            let top = format!(
                "{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
                x, y - height,
                x + cell * 0.5, y - height + cell * 0.3,
                x, y - height + cell * 0.6,
                x - cell * 0.5, y - height + cell * 0.3,
            );
            s.push_str(&format!(
                r#"  <polygon points="{}" fill="{}" opacity="{:.2}"/>"#,
                top, color, opacity
            ));
            s.push('\n');

            let left = format!(
                "{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
                x - cell * 0.5, y - height + cell * 0.3,
                x, y - height + cell * 0.6,
                x, y + cell * 0.6 - height + height,
                x - cell * 0.5, y + cell * 0.3 - height + height,
            );
            s.push_str(&format!(
                r#"  <polygon points="{}" fill="{}" opacity="{:.2}"/>"#,
                left, color, opacity * 0.7
            ));
            s.push('\n');
        }
    }
    s
}

fn gen_abstract(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let mut s = String::new();
    let count = rng.gen_range(6..14);

    for _ in 0..count {
        let color = pick(rng, palette);
        let opacity = rng.gen_range(0.15..0.6);

        let cx = rng.gen_range(0.0..w);
        let cy = rng.gen_range(0.0..h);
        let r = rng.gen_range(w * 0.05..w * 0.25);
        let points_count = rng.gen_range(4..8);

        let mut path = String::new();
        let angles: Vec<f64> = (0..points_count)
            .map(|i| (i as f64 / points_count as f64) * PI * 2.0)
            .collect();

        for (i, &angle) in angles.iter().enumerate() {
            let rr = r * rng.gen_range(0.6..1.4);
            let px = cx + angle.cos() * rr;
            let py = cy + angle.sin() * rr;

            if i == 0 {
                path.push_str(&format!("M {:.1} {:.1}", px, py));
            } else {
                let prev_angle = angles[i - 1];
                let prev_rr = r * rng.gen_range(0.6..1.4);
                let cp1x = cx + prev_angle.cos() * prev_rr * 1.3;
                let cp1y = cy + prev_angle.sin() * prev_rr * 1.3;
                let cp2x = cx + angle.cos() * rr * 1.3;
                let cp2y = cy + angle.sin() * rr * 1.3;
                path.push_str(&format!(
                    " C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}",
                    cp1x, cp1y, cp2x, cp2y, px, py
                ));
            }
        }
        path.push_str(" Z");

        s.push_str(&format!(
            r#"  <path d="{}" fill="{}" opacity="{:.2}"/>"#,
            path, color, opacity
        ));
        s.push('\n');
    }
    s
}

// --- Sunburst: radiating rays and concentric arcs ---

fn gen_sunburst(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let style = rng.gen_range(0..3);
    match style {
        0 => gen_sunburst_radiating(rng, palette.colors, w, h),
        1 => gen_sunburst_concentric(rng, palette.colors, w, h),
        _ => {
            let mut s = gen_sunburst_concentric(rng, palette.colors, w, h);
            s.push_str(&gen_sunburst_radiating(rng, palette.colors, w, h));
            s
        }
    }
}

fn gen_sunburst_radiating(rng: &mut impl Rng, colors: &[&str], w: f64, h: f64) -> String {
    let mut s = String::new();

    let origins: [(f64, f64); 5] = [
        (w * 0.5, h * 1.0),
        (0.0, h),
        (w, h),
        (w * 0.5, h * 0.5),
        (0.0, h * 0.5),
    ];
    let (ox, oy) = origins[rng.gen_range(0..origins.len())];

    let ray_count = rng.gen_range(12..22);
    let reach = (w * w + h * h).sqrt() * 1.2;

    for i in 0..ray_count {
        let color = colors[i % colors.len()];
        let a1 = (i as f64 / ray_count as f64) * PI * 2.0;
        let a2 = ((i as f64 + 0.5) / ray_count as f64) * PI * 2.0;

        let x1 = ox + a1.cos() * reach;
        let y1 = oy + a1.sin() * reach;
        let x2 = ox + a2.cos() * reach;
        let y2 = oy + a2.sin() * reach;

        s.push_str(&format!(
            r#"  <polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" fill="{}" opacity="0.85"/>"#,
            ox, oy, x1, y1, x2, y2, color
        ));
        s.push('\n');
    }
    s
}

fn gen_sunburst_concentric(rng: &mut impl Rng, colors: &[&str], w: f64, h: f64) -> String {
    let mut s = String::new();

    let focal_count = rng.gen_range(2..4);
    let band_width = rng.gen_range(w * 0.025..w * 0.05);
    let max_r = (w * w + h * h).sqrt() * 0.5;

    for f in 0..focal_count {
        let positions: [(f64, f64); 6] = [
            (-w * 0.1, -h * 0.1),
            (w * 1.1, -h * 0.1),
            (-w * 0.1, h * 1.1),
            (w * 1.1, h * 1.1),
            (w * 0.5, -h * 0.2),
            (w * 0.5, h * 1.2),
        ];
        let (fx, fy) = positions[rng.gen_range(0..positions.len())];
        let ring_count = (max_r / band_width) as i32;

        for r_i in 0..ring_count {
            let r = r_i as f64 * band_width;
            let color_idx = (f + r_i as usize) % colors.len();
            let color = colors[color_idx];

            s.push_str(&format!(
                r#"  <circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="none" stroke="{}" stroke-width="{:.1}" opacity="0.85"/>"#,
                fx, fy, r, color, band_width
            ));
            s.push('\n');
        }
    }
    s
}

// --- Wave generators ---

fn wave_y(x: f64, w: f64, base_y: f64, amplitude: f64, frequency: f64, phase: f64) -> f64 {
    let t = x / w;
    base_y
        + (t * PI * 2.0 * frequency + phase).sin() * amplitude
        + (t * PI * 2.0 * frequency * 0.6 + phase * 1.7).sin() * amplitude * 0.4
        + (t * PI * 2.0 * frequency * 2.1 + phase * 0.3).sin() * amplitude * 0.15
}

fn gen_waves(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    gen_waves_inner(rng, palette.colors, w, h)
}

fn gen_waves_inner(rng: &mut impl Rng, colors: &[&str], w: f64, h: f64) -> String {
    let mut s = String::new();
    let segments = 80;

    let frequency = rng.gen_range(1.2..2.8);
    let phase = rng.gen_range(0.0..PI * 2.0);
    let base_amplitude = rng.gen_range(h * 0.1..h * 0.25);
    let center_y = h * rng.gen_range(0.35..0.65);

    let band_count = rng.gen_range(8..16);
    let total_thickness = base_amplitude * rng.gen_range(1.2..2.0);

    for i in 0..band_count {
        let color = colors[i % colors.len()];
        let t = i as f64 / (band_count - 1) as f64;

        let spread = (0.5 - t).abs() * 2.0;
        let band_offset = (t - 0.5) * total_thickness;
        let band_amp = base_amplitude * (1.0 + spread * 0.3);
        let band_phase = phase + (t - 0.5) * rng.gen_range(0.1..0.4);
        let band_base = center_y + band_offset;

        let thickness = total_thickness / band_count as f64
            * rng.gen_range(0.8..1.4);

        let mut upper_points: Vec<(f64, f64)> = Vec::new();
        let mut lower_points: Vec<(f64, f64)> = Vec::new();

        for j in 0..=segments {
            let x = w * j as f64 / segments as f64;
            let cy = wave_y(x, w, band_base, band_amp, frequency, band_phase);
            upper_points.push((x, cy - thickness * 0.5));
            lower_points.push((x, cy + thickness * 0.5));
        }

        let mut path = format!("M {:.1} {:.1}", upper_points[0].0, upper_points[0].1);
        for &(x, y) in &upper_points[1..] {
            path.push_str(&format!(" L {:.1} {:.1}", x, y));
        }
        for &(x, y) in lower_points.iter().rev() {
            path.push_str(&format!(" L {:.1} {:.1}", x, y));
        }
        path.push_str(" Z");

        let opacity = rng.gen_range(0.7..0.95);
        s.push_str(&format!(
            r#"  <path d="{}" fill="{}" opacity="{:.2}"/>"#,
            path, color, opacity
        ));
        s.push('\n');
    }

    let accent_count = rng.gen_range(3..6);
    for a in 0..accent_count {
        let t = (a as f64 + 0.5) / accent_count as f64;
        let band_offset = (t - 0.5) * total_thickness;
        let band_phase = phase + (t - 0.5) * 0.2;
        let band_base = center_y + band_offset;
        let color = colors[a % colors.len()];

        let mut path = String::new();
        for j in 0..=segments {
            let x = w * j as f64 / segments as f64;
            let y = wave_y(x, w, band_base, base_amplitude, frequency, band_phase);
            if j == 0 {
                path.push_str(&format!("M {:.1} {:.1}", x, y));
            } else {
                path.push_str(&format!(" L {:.1} {:.1}", x, y));
            }
        }

        s.push_str(&format!(
            r#"  <path d="{}" fill="none" stroke="{}" stroke-width="1.5" opacity="0.4"/>"#,
            path, color
        ));
        s.push('\n');
    }

    s
}

fn gen_silkflow(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let mut defs = String::new();
    let mut shapes = String::new();

    let ci = rng.gen_range(0..palette.colors.len());
    let colors: Vec<&str> = (0..3)
        .map(|j| palette.colors[(ci + j) % palette.colors.len()])
        .collect();

    let ribbon_count = rng.gen_range(6..10);
    let base_flow = rng.gen_range(0.0..PI * 2.0);

    for i in 0..ribbon_count {
        let color = colors[i % colors.len()];
        let grad_id = format!("silk{}", i);

        let ribbon_width = rng.gen_range(w * 0.06..w * 0.18);

        let flow_angle = base_flow + rng.gen_range(-0.6..0.6);
        let perp = flow_angle + PI / 2.0;

        let offset = (i as f64 - ribbon_count as f64 / 2.0) * ribbon_width * 0.6;
        let cx = w * 0.5 + perp.cos() * offset;
        let cy = h * 0.5 + perp.sin() * offset;

        let reach = (w * w + h * h).sqrt() * 0.6;
        let sx = cx - flow_angle.cos() * reach;
        let sy = cy - flow_angle.sin() * reach;
        let ex = cx + flow_angle.cos() * reach;
        let ey = cy + flow_angle.sin() * reach;

        let bend1 = rng.gen_range(-w * 0.3..w * 0.3);
        let bend2 = rng.gen_range(-w * 0.3..w * 0.3);
        let t1 = 0.33;
        let t2 = 0.66;
        let cp1x = sx + (ex - sx) * t1 + perp.cos() * bend1;
        let cp1y = sy + (ey - sy) * t1 + perp.sin() * bend1;
        let cp2x = sx + (ex - sx) * t2 + perp.cos() * bend2;
        let cp2y = sy + (ey - sy) * t2 + perp.sin() * bend2;

        let hw = ribbon_width * 0.5;
        let hw_start = hw * rng.gen_range(0.4..1.0);
        let hw_mid1 = hw * rng.gen_range(0.8..1.5);
        let hw_mid2 = hw * rng.gen_range(0.8..1.5);
        let hw_end = hw * rng.gen_range(0.4..1.0);

        let us_x = sx + perp.cos() * hw_start;
        let us_y = sy + perp.sin() * hw_start;
        let uc1_x = cp1x + perp.cos() * hw_mid1;
        let uc1_y = cp1y + perp.sin() * hw_mid1;
        let uc2_x = cp2x + perp.cos() * hw_mid2;
        let uc2_y = cp2y + perp.sin() * hw_mid2;
        let ue_x = ex + perp.cos() * hw_end;
        let ue_y = ey + perp.sin() * hw_end;

        let ls_x = sx - perp.cos() * hw_start;
        let ls_y = sy - perp.sin() * hw_start;
        let lc1_x = cp1x - perp.cos() * hw_mid1;
        let lc1_y = cp1y - perp.sin() * hw_mid1;
        let lc2_x = cp2x - perp.cos() * hw_mid2;
        let lc2_y = cp2y - perp.sin() * hw_mid2;
        let le_x = ex - perp.cos() * hw_end;
        let le_y = ey - perp.sin() * hw_end;

        let path = format!(
            "M {:.1} {:.1} C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1} \
             L {:.1} {:.1} C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1} Z",
            us_x, us_y, uc1_x, uc1_y, uc2_x, uc2_y, ue_x, ue_y,
            le_x, le_y, lc2_x, lc2_y, lc1_x, lc1_y, ls_x, ls_y,
        );

        let gx1 = (sx / w * 100.0).clamp(0.0, 100.0);
        let gy1 = (sy / h * 100.0).clamp(0.0, 100.0);
        let gx2 = (ex / w * 100.0).clamp(0.0, 100.0);
        let gy2 = (ey / h * 100.0).clamp(0.0, 100.0);
        let next_color = colors[(i + 1) % colors.len()];

        defs.push_str(&format!(
            r#"    <linearGradient id="{}" x1="{:.0}%" y1="{:.0}%" x2="{:.0}%" y2="{:.0}%">
      <stop offset="0%" stop-color="{}" stop-opacity="0.1"/>
      <stop offset="30%" stop-color="{}" stop-opacity="0.6"/>
      <stop offset="70%" stop-color="{}" stop-opacity="0.6"/>
      <stop offset="100%" stop-color="{}" stop-opacity="0.1"/>
    </linearGradient>
"#,
            grad_id, gx1, gy1, gx2, gy2, color, color, next_color, next_color
        ));

        let opacity = rng.gen_range(0.25..0.55);
        shapes.push_str(&format!(
            r#"  <path d="{}" fill="url(#{})" opacity="{:.2}"/>"#,
            path, grad_id, opacity
        ));
        shapes.push('\n');
    }

    let glow_id = "silkGlow";
    let gc = colors[0];
    defs.push_str(&format!(
        r#"    <radialGradient id="{}" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="{}" stop-opacity="0.4"/>
      <stop offset="100%" stop-color="{}" stop-opacity="0"/>
    </radialGradient>
"#,
        glow_id, gc, gc
    ));
    shapes.push_str(&format!(
        r#"  <ellipse cx="{:.1}" cy="{:.1}" rx="{:.1}" ry="{:.1}" fill="url(#{})"/>"#,
        w * 0.5, h * 0.5, w * 0.5, h * 0.5, glow_id
    ));
    shapes.push('\n');

    format!("  <defs>\n{}</defs>\n{}", defs, shapes)
}

fn gen_dots(rng: &mut impl Rng, palette: &Palette, w: f64, h: f64) -> String {
    let mut s = String::new();
    let spacing = rng.gen_range(w * 0.04..w * 0.08);
    let base_r = spacing * 0.2;
    let cols = (w / spacing) as i32 + 1;
    let rows = (h / spacing) as i32 + 1;

    let focus_x = rng.gen_range(w * 0.2..w * 0.8);
    let focus_y = rng.gen_range(h * 0.2..h * 0.8);
    let max_dist = (w * w + h * h).sqrt() * 0.5;

    for row in 0..rows {
        for col in 0..cols {
            let x = col as f64 * spacing + spacing * 0.5;
            let y = row as f64 * spacing + spacing * 0.5;
            let dist = ((x - focus_x).powi(2) + (y - focus_y).powi(2)).sqrt();
            let scale = 1.0 - (dist / max_dist).min(1.0);
            let r = base_r * (0.3 + scale * 1.5);
            let color = pick(rng, palette);
            let opacity = 0.2 + scale * 0.5;

            s.push_str(&format!(
                r#"  <circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="{}" opacity="{:.2}"/>"#,
                x, y, r, color, opacity
            ));
            s.push('\n');
        }
    }
    s
}
