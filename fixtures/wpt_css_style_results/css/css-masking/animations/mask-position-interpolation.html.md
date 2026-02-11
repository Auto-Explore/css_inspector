# css/css-masking/animations/mask-position-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/animations/mask-position-interpolation.html"
}
```

## style[0]

```css

.container {
  display: inline-block;
  border: 2px solid black;
}
.parent {
  mask-position: 30px 10px;
}
.target {
  width: 120px;
  height: 120px;
  mask-position: 10px 30px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
