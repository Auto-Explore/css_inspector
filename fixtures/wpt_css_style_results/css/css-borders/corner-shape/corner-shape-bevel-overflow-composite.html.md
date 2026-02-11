# css/css-borders/corner-shape/corner-shape-bevel-overflow-composite.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-bevel-overflow-composite.html"
}
```

## style[0]

```css

    .bevel {
        width: 100px;
        height: 100px;
        border-radius: 50px;
        border: 0px solid black;
        corner-shape: bevel;
        overflow: clip;
        transform: translate3d(10px, 10px, 10px);
        transform-style: preserve-3d;
    }

    .green {
        will-change: transform;
        width: 200px;
        height: 200px;
        background: green;
        transform: translate3d(10px, 10px, 10px);
        transform-style: preserve-3d;
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
