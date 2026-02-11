# css/css-paint-api/non-registered-property-value.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/non-registered-property-value.https.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
}

#canvas-geometry {
  --foo:100;
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
