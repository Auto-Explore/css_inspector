# css/css-borders/corner-shape/corner-shape-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-interpolation.html"
}
```

## style[0]

```css

.parent {
  corner-shape: bevel;
  padding: 10px;
}

.target {
  display: inline-block;
  width: 100px;
  height: 100px;
  background-color: black;
  corner-shape: bevel superellipse(3) superellipse(-2.5) square;
}

.expected {
  background-color: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “corner-shape”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “corner-shape”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
