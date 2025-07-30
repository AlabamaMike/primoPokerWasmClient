# Primo Poker Assets

This directory contains all visual assets for the Primo Poker WebAssembly client.

## Directory Structure

### `/logos/`
- `primo-poker-logo.svg` - Main horizontal logo with text
- `primo-poker-icon.svg` - Icon-only version for favicons and small spaces
- `primo-poker-wordmark.svg` - Text-only version (to be created)
- `primo-poker-compact.svg` - Compact version for mobile headers (to be created)

### `/backgrounds/`
- Poker table textures
- Casino ambient backgrounds
- Wood grain patterns
- (To be populated with actual texture files)

### `/ui/`
- `card-suits.svg` - Hearts, diamonds, clubs, spades icons
- `chip-stack.svg` - Poker chip representation
- `dealer-button.svg` - Dealer button graphic (to be created)
- `loading-cards.svg` - Animated loading graphic (to be created)
- `trophy-icon.svg` - Winner indicators (to be created)

### `/textures/`
- Felt textures for table surfaces
- Leather trim textures
- Metallic shine overlays
- (To be populated with texture files)

### `/patterns/`
- Repeating patterns for backgrounds
- Subtle texture overlays
- (To be populated with pattern files)

## Usage Guidelines

### Logo Usage
- Use `primo-poker-logo.svg` for main branding in headers
- Use `primo-poker-icon.svg` for favicons and app icons
- Maintain minimum size of 32px for legibility
- Ensure proper contrast against backgrounds

### File Formats
- **SVG**: For scalable graphics (logos, icons, patterns)
- **WebP**: For photographs and complex images (with JPEG fallback)
- **PNG**: For images requiring transparency
- **JPEG**: For large background images without transparency

### Performance Guidelines
- Keep individual files under 500KB
- Optimize SVGs by removing unnecessary metadata
- Use appropriate compression for raster images
- Consider lazy loading for non-critical assets

## Asset Creation Standards

### Colors
- Primary Green: #0f5132
- Accent Gold: #ffc107
- Felt Background: #1a2e1a
- Wood Brown: #8b4513

### Styling
- Consistent with poker/casino theme
- Professional and premium feel
- Accessible contrast ratios
- Responsive across all screen sizes

## Integration

Assets are referenced in CSS and components using relative paths:
```css
background-image: url('/assets/backgrounds/poker-table-felt.jpg');
```

```html
<img src="/assets/logos/primo-poker-logo.svg" alt="Primo Poker" />
```
