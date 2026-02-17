#!/usr/bin/env python3
"""Generate WallCraft app icon — retro horizontal stripes with block text."""

from PIL import Image, ImageDraw
import os

SIZE = 1024
OUT_DIR = os.path.join(os.path.dirname(__file__), '..', 'src-tauri', 'icons')
PREVIEW_DIR = os.path.dirname(__file__)


def draw_rounded_rect(draw, bbox, radius, fill):
    x0, y0, x1, y1 = bbox
    r = min(radius, (x1 - x0) // 2, (y1 - y0) // 2)
    if r <= 0:
        draw.rectangle(bbox, fill=fill)
        return
    draw.pieslice([x0, y0, x0 + 2 * r, y0 + 2 * r], 180, 270, fill=fill)
    draw.pieslice([x1 - 2 * r, y0, x1, y0 + 2 * r], 270, 360, fill=fill)
    draw.pieslice([x0, y1 - 2 * r, x0 + 2 * r, y1], 90, 180, fill=fill)
    draw.pieslice([x1 - 2 * r, y1 - 2 * r, x1, y1], 0, 90, fill=fill)
    draw.rectangle([x0 + r, y0, x1 - r, y1], fill=fill)
    draw.rectangle([x0, y0 + r, x0 + r, y1 - r], fill=fill)
    draw.rectangle([x1 - r, y0 + r, x1, y1 - r], fill=fill)


def draw_block_letter(draw, letter, x, y, w, h, color):
    """Draw a single retro block letter. Each letter is drawn as filled rectangles
    to create a chunky, geometric, retro-style typeface."""
    t = max(int(w * 0.24), 2)  # stroke thickness
    letter = letter.upper()

    if letter == 'W':
        # Two down-strokes on sides, two inner diagonals meeting at bottom
        # Simplified as: left bar, right bar, two inner bars forming V shape
        leg = w // 5
        draw.rectangle([x, y, x + t, y + h], fill=color)  # left
        draw.rectangle([x + w - t, y, x + w, y + h], fill=color)  # right
        # Inner left leg
        ix1 = x + leg + t // 2
        draw.rectangle([ix1, y + h // 3, ix1 + t, y + h], fill=color)
        # Inner right leg
        ix2 = x + w - leg - t - t // 2
        draw.rectangle([ix2, y + h // 3, ix2 + t, y + h], fill=color)
        # Bottom connector
        draw.rectangle([x, y + h - t, x + w, y + h], fill=color)
        # Mid connector
        draw.rectangle([ix1, y + h // 2, ix2 + t, y + h // 2 + t], fill=color)

    elif letter == 'A':
        draw.rectangle([x, y, x + t, y + h], fill=color)  # left
        draw.rectangle([x + w - t, y, x + w, y + h], fill=color)  # right
        draw.rectangle([x, y, x + w, y + t], fill=color)  # top
        # crossbar at mid
        mid_y = y + h // 2
        draw.rectangle([x, mid_y, x + w, mid_y + t], fill=color)

    elif letter == 'L':
        draw.rectangle([x, y, x + t, y + h], fill=color)  # left
        draw.rectangle([x, y + h - t, x + w, y + h], fill=color)  # bottom

    elif letter == 'C':
        draw.rectangle([x, y, x + t, y + h], fill=color)  # left
        draw.rectangle([x, y, x + w, y + t], fill=color)  # top
        draw.rectangle([x, y + h - t, x + w, y + h], fill=color)  # bottom

    elif letter == 'R':
        draw.rectangle([x, y, x + t, y + h], fill=color)  # left
        draw.rectangle([x, y, x + w, y + t], fill=color)  # top
        draw.rectangle([x + w - t, y, x + w, y + h // 2 + t], fill=color)  # right upper
        draw.rectangle([x, y + h // 2, x + w, y + h // 2 + t], fill=color)  # mid bar
        # Diagonal leg: approximate with a slanted rectangle
        leg_steps = h // 2
        for i in range(leg_steps):
            frac = i / max(leg_steps - 1, 1)
            lx = int(x + w * 0.3 + frac * (w * 0.7 - t))
            ly = y + h // 2 + t + i
            if ly + t > y + h:
                break
            draw.rectangle([lx, ly, lx + t, ly + 1], fill=color)

    elif letter == 'F':
        draw.rectangle([x, y, x + t, y + h], fill=color)  # left
        draw.rectangle([x, y, x + w, y + t], fill=color)  # top
        draw.rectangle([x, y + h // 2, x + int(w * 0.75), y + h // 2 + t], fill=color)  # mid

    elif letter == 'T':
        draw.rectangle([x, y, x + w, y + t], fill=color)  # top
        cx = x + w // 2 - t // 2
        draw.rectangle([cx, y, cx + t, y + h], fill=color)  # center vertical


def measure_text(text, char_w, spacing):
    """Return total width for a string of block letters."""
    if not text:
        return 0
    return len(text) * char_w + (len(text) - 1) * spacing


def draw_text_line(draw, text, cx, y, char_w, char_h, spacing, color):
    """Draw centered block text."""
    total_w = measure_text(text, char_w, spacing)
    start_x = cx - total_w // 2
    for i, ch in enumerate(text):
        lx = start_x + i * (char_w + spacing)
        draw_block_letter(draw, ch, lx, y, char_w, char_h, color)


def draw_text_left(draw, text, x, y, char_w, char_h, spacing, color):
    """Draw left-aligned block text."""
    for i, ch in enumerate(text):
        lx = x + i * (char_w + spacing)
        draw_block_letter(draw, ch, lx, y, char_w, char_h, color)


def generate_icon():
    S = SIZE
    img = Image.new('RGBA', (S, S), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    margin = int(S * 0.02)
    corner_r = int(S * 0.18)

    # Background
    draw_rounded_rect(draw, (margin, margin, S - margin, S - margin), corner_r, (28, 28, 28, 255))

    # --- Stripe colors (6 stripes, warm retro gradient) ---
    stripe_colors = [
        (210, 60, 50),    # red
        (230, 130, 50),   # orange
        (240, 195, 55),   # yellow
        (160, 200, 80),   # yellow-green
        (60, 175, 160),   # teal
        (50, 130, 170),   # blue-teal
    ]

    # Stripe geometry
    stripe_left = margin + int(S * 0.08)
    stripe_right = S - margin - int(S * 0.08)
    stripe_top = margin + int(S * 0.08)
    stripe_zone_h = int(S * 0.38)  # total height for all stripes
    n = len(stripe_colors)
    gap = int(S * 0.018)  # gap between stripes
    stripe_h = (stripe_zone_h - (n - 1) * gap) // n

    for i, color in enumerate(stripe_colors):
        sy = stripe_top + i * (stripe_h + gap)
        sr = int(S * 0.012)  # slight rounding on stripes
        draw_rounded_rect(draw, (stripe_left, sy, stripe_right, sy + stripe_h), sr, (*color, 255))

    # --- Text: "WALL" / "CRAFT" — aligned to stripe edges ---
    text_color = (235, 230, 210, 255)
    text_top = stripe_top + stripe_zone_h + int(S * 0.05)

    # Both lines span from stripe_left to stripe_right
    text_span = stripe_right - stripe_left
    char_h = int(S * 0.17)  # slightly smaller letters

    # "CRAFT" = 5 chars, fill text_span with spacing ~ 0.35 * char_w
    craft_char_w = int(text_span / (5 + 4 * 0.35))
    craft_spacing = (text_span - 5 * craft_char_w) // 4

    # "WALL" = 4 chars, same char width, spacing fills the rest
    wall_char_w = craft_char_w
    wall_spacing = (text_span - 4 * wall_char_w) // 3

    wall_y = text_top
    craft_y = text_top + char_h + int(S * 0.03)

    draw_text_left(draw, "WALL", stripe_left, wall_y, wall_char_w, char_h, wall_spacing, text_color)
    draw_text_left(draw, "CRAFT", stripe_left, craft_y, craft_char_w, char_h, craft_spacing, text_color)

    # Clip everything to the rounded rect mask
    mask = Image.new('L', (S, S), 0)
    md = ImageDraw.Draw(mask)
    draw_rounded_rect(md, (margin, margin, S - margin, S - margin), corner_r, 255)
    img.putalpha(mask)

    return img


def save_all_sizes(master):
    os.makedirs(OUT_DIR, exist_ok=True)

    png_sizes = {
        '32x32.png': 32,
        '128x128.png': 128,
        '128x128@2x.png': 256,
        'icon.png': 512,
        'Square30x30Logo.png': 30,
        'Square44x44Logo.png': 44,
        'Square71x71Logo.png': 71,
        'Square89x89Logo.png': 89,
        'Square107x107Logo.png': 107,
        'Square142x142Logo.png': 142,
        'Square150x150Logo.png': 150,
        'Square284x284Logo.png': 284,
        'Square310x310Logo.png': 310,
        'StoreLogo.png': 50,
    }

    for filename, size in png_sizes.items():
        resized = master.resize((size, size), Image.LANCZOS)
        path = os.path.join(OUT_DIR, filename)
        resized.save(path, 'PNG')
        print(f"  Saved {filename} ({size}x{size})")

    # ICO (Windows)
    ico_sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    ico_images = [master.resize(s, Image.LANCZOS) for s in ico_sizes]
    ico_path = os.path.join(OUT_DIR, 'icon.ico')
    ico_images[0].save(ico_path, format='ICO', sizes=ico_sizes, append_images=ico_images[1:])
    print(f"  Saved icon.ico (multi-size)")

    # ICNS (macOS)
    icns_path = os.path.join(OUT_DIR, 'icon.icns')
    try:
        master.save(icns_path, format='ICNS')
        print(f"  Saved icon.icns")
    except Exception as e:
        print(f"  ICNS save failed ({e}), skipping")

    # GNOME hicolor
    home = os.path.expanduser('~')
    for dirname, size in {'48x48': 48, '64x64': 64, '96x96': 96, '128x128': 128, '256x256': 256, '512x512': 512}.items():
        dest_dir = os.path.join(home, '.local', 'share', 'icons', 'hicolor', dirname, 'apps')
        os.makedirs(dest_dir, exist_ok=True)
        dest = os.path.join(dest_dir, 'wallcraft.png')
        master.resize((size, size), Image.LANCZOS).save(dest, 'PNG')
        print(f"  Installed {dirname}/apps/wallcraft.png")

    hicolor_base = os.path.join(home, '.local', 'share', 'icons', 'hicolor')
    os.system(f'gtk-update-icon-cache -f -t "{hicolor_base}" 2>/dev/null')
    os.system(f'update-desktop-database "{os.path.join(home, ".local/share/applications")}" 2>/dev/null')


def save_preview(master, name="preview_R4b.png"):
    """Save a 256x256 preview."""
    preview = master.resize((256, 256), Image.LANCZOS)
    path = os.path.join(PREVIEW_DIR, name)
    preview.save(path, 'PNG')
    print(f"  Preview saved: {path}")
    # Also save a larger version for better viewing
    preview_lg = master.resize((512, 512), Image.LANCZOS)
    path_lg = os.path.join(PREVIEW_DIR, name.replace('.png', '_512.png'))
    preview_lg.save(path_lg, 'PNG')
    print(f"  Preview (large) saved: {path_lg}")


if __name__ == '__main__':
    import sys
    print("Generating WallCraft retro stripe icon...")
    icon = generate_icon()

    if '--preview' in sys.argv:
        save_preview(icon)
    else:
        print("Saving all sizes...")
        save_all_sizes(icon)
        save_preview(icon)
        print("Done!")
