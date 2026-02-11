# css/css-borders/corner-shape/corner-shape-square.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-square.html"
}
```

## style[0]

```css

    .target {
        background: green;
        width: 100px;
        height: 100px;
        border-radius: 25px;
        border: 10px solid black;
        box-sizing: border-box;
        corner-shape: square superellipse(24) round square;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “corner-shape”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
