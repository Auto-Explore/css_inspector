# css/motion/offset-distance-008.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-distance-008.html"
}
```

## style[0]

```css

      #target {
        position: absolute;
        width: 100px;
        height: 40px;
        background-color: lime;
        transform-origin: 0% 0%;
        offset-path: path('m -80 0 h 200');
        offset-distance: 120%;
      }
    
```

```json
{
  "errors": 3,
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
    }
  ],
  "warnings": 0
}
```
