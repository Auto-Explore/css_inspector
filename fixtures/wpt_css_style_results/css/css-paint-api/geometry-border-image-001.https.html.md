# css/css-paint-api/geometry-border-image-001.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/geometry-border-image-001.https.html"
}
```

## style[0]

```css

html, body { margin: 0; padding: 0; }
.container {
  width: 200px;
  height: 200px;
}

#canvas-geometry {
  border: solid 0;
  border-image: paint(geometry);
  border-image-slice: 0 fill;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-image-slice”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
