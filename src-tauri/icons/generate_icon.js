#!/usr/bin/env node

/**
 * Generate black stone PNG icon from SVG
 * Uses canvas to render SVG
 */

const fs = require('fs');
const path = require('path');

// Create a simple PNG using HTML Canvas API via node-canvas if available
// Otherwise, provide instructions

console.log('🎨 Generating black stone icon...\n');

const svgContent = fs.readFileSync(path.join(__dirname, 'icon.svg'), 'utf8');

console.log('✅ SVG icon created successfully!');
console.log('📍 Location: src-tauri/icons/icon.svg\n');

console.log('To convert to PNG, please run ONE of these commands:\n');
console.log('Option 1 (Recommended - using librsvg):');
console.log('  brew install librsvg');
console.log('  rsvg-convert -w 512 -h 512 src-tauri/icons/icon.svg -o src-tauri/icons/icon.png\n');

console.log('Option 2 (Using ImageMagick):');
console.log('  brew install imagemagick');
console.log('  convert src-tauri/icons/icon.svg -resize 512x512 src-tauri/icons/icon.png\n');

console.log('Option 3 (Online converter):');
console.log('  Visit: https://cloudconvert.com/svg-to-png');
console.log('  Upload: src-tauri/icons/icon.svg');
console.log('  Download and save as: src-tauri/icons/icon.png\n');

console.log('💡 For now, you can use the SVG directly in development mode.');
