# css/motion/offset-path-ray-009.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-009.html"
}
```

## style[0]

```css

      #container {
        width: 200px;
        height: 200px;
      }
      #target {
        position: relative;
        left: 140%;
        top: 70%;
        width: 40px;
        height: 40px;
        background-color: lime;
        offset-path: ray(180deg closest-side);
        offset-distance: 100%;
        offset-position: auto;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
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
