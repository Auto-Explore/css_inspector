# css/motion/offset-path-ray-contain-001.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-contain-001.html"
}
```

## style[0]

```css

      #container {
        width: 300px;
        height: 300px;
      }
      #target {
        position: relative;
        left: 30px;
        top: 40px;
        width: 10px;
        height: 10px;
        background-color: lime;
        offset-path: ray(180deg closest-corner contain);
        offset-rotate: 0deg;
        offset-distance: 100%;
        offset-position: auto;
      }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-rotate”.",
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
