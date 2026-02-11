# css/motion/offset-distance-001.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-distance-001.html"
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
        offset-path: path('m 0 0 h 200 v 150 z');
        offset-distance: 20%;
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
