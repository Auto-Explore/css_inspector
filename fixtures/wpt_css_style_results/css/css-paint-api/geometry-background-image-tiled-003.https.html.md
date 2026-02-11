# css/css-paint-api/geometry-background-image-tiled-003.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/geometry-background-image-tiled-003.https.html"
}
```

## style[0]

```css

html, body { margin: 0; padding: 0; }
.container {
  width: 100px;
  height: 100px;
  background: paint(geometry) center center/60px 80px no-repeat;
}

#canvas-geometry {
  background-image: paint(geometry);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
