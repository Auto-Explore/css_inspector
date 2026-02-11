# css/motion/offset-path-ray-001.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-001.html"
}
```

## style[0]

```css

      #target {
        position: absolute;
        left: 300px;
        top: 100px;
        width: 300px;
        height: 200px;
        background-color: lime;
        transform-origin: 0px 0px;
        offset-path: ray(135deg closest-side);
        offset-distance: 20px;
        offset-position: auto;
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
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-distance”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
