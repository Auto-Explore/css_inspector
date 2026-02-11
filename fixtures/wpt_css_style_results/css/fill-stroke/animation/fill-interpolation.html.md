# css/fill-stroke/animation/fill-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/fill-stroke/animation/fill-interpolation.html"
}
```

## style[0]

```css

.parent {
  fill: blue;
}
.target {
  display: inline-block;
  font-size: 60pt;
  fill: yellow;
}
.expected {
  margin-right: 15px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
