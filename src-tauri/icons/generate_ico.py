#!/usr/bin/env python3
"""
Generate Windows .ico file from PNG
"""
from PIL import Image
import sys

def generate_ico(png_path, ico_path):
    """Convert PNG to ICO with multiple sizes"""
    img = Image.open(png_path)

    # ICO format supports multiple sizes
    # Windows commonly uses: 16, 32, 48, 256
    sizes = [(16, 16), (32, 32), (48, 48), (256, 256)]

    # Create images at different sizes
    icon_sizes = []
    for size in sizes:
        resized = img.resize(size, Image.Resampling.LANCZOS)
        icon_sizes.append(resized)

    # Save as ICO
    icon_sizes[0].save(
        ico_path,
        format='ICO',
        sizes=sizes,
        append_images=icon_sizes[1:]
    )
    print(f"✓ Created {ico_path}")

if __name__ == '__main__':
    generate_ico('icon.png', 'icon.ico')
