# css/css-paint-api/geometry-background-image-tiled-001.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/geometry-background-image-tiled-001.https.html"
}
```

## style[0]

```css

html, body { margin: 0; padding: 0; }
.container {
  width: 100px;
  height: 100px;
  background: paint(geometry) top left/50% 50% repeat-x;
}

#canvas-geometry {
  background-image: paint(geometry);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
