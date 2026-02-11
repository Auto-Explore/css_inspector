# css/css-paint-api/parse-input-arguments-020.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/parse-input-arguments-020.https.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
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
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
