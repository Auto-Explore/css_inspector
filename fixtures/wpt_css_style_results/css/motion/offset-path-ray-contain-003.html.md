# css/motion/offset-path-ray-contain-003.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-contain-003.html"
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
        left: 150px;
        top: 150px;
        width: 100px;
        height: 100px;
        background-color: lime;
        offset-path: ray(45deg closest-side contain);
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
