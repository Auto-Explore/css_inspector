# css/css-paint-api/geometry-border-image-005.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/geometry-border-image-005.https.html"
}
```

## style[0]

```css

html, body { margin: 0; padding: 0; }
.container {
  width: 200px;
  height: 200px;
  border: 36px solid orange;
  border-image: paint(foo)               /* source */
                1 /                      /* slice */
                36px 36px 36px 36px /    /* width */
                18px 18px 18px 18px      /* outset */
                round;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
