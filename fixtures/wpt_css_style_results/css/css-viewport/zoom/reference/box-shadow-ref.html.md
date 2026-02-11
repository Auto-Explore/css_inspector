# css/css-viewport/zoom/reference/box-shadow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/box-shadow-ref.html"
}
```

## style[0]

```css

body {
  --scale: 1;
  --shadowScaleOffsetX: 1;
  --shadowScaleOffsetY: 1;
  --shadowScaleBlurRadius: 1;
}
div {
  font-size: calc(1rem * var(--scale));
  box-shadow: calc(5px * var(--shadowScaleOffsetX)) calc(5px * var(--shadowScaleOffsetY)) calc(2px * var(--shadowScaleBlurRadius)) black;
}
.zoomFont {
  --scale: 2;
}
.zoomShadow {
  --shadowScaleOffsetX: 2;
  --shadowScaleOffsetY: 2;
  --shadowScaleBlurRadius: 2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
