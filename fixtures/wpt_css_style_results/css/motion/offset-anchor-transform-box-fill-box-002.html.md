# css/motion/offset-anchor-transform-box-fill-box-002.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-anchor-transform-box-fill-box-002.html"
}
```

## style[0]

```css

#target {
  transform-box: fill-box;
  transform-origin: 50% 50%;
  offset-anchor: 25% 25%;
  offset-path: path("M75,-25v100");
  offset-distance: 50%;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-distance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
