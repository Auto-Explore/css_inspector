# css/motion/offset-path-ray-005.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-005.html"
}
```

## style[0]

```css

      #container {
        width: 400px;
        height: 400px;
      }
      #target {
        position: relative;
        left: 120px;
        top: 160px;
        width: 100px;
        height: 50px;
        background-color: lime;
        transform-origin: 0px 0px;
        offset-path: ray(135deg closest-corner);
        offset-distance: 100%;
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
