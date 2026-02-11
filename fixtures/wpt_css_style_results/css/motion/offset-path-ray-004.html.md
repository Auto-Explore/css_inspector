# css/motion/offset-path-ray-004.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-004.html"
}
```

## style[0]

```css

      #container {
        width: 150px;
        height: 150px;
      }
      #target {
        position: relative;
        left: 100px;
        top: 100px;
        width: 100px;
        height: 50px;
        background-color: lime;
        transform-origin: 0px 0px;
        offset-path: ray(135deg farthest-side);
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
