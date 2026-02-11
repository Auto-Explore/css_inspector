# css/motion/offset-rotate-002.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-rotate-002.html"
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
        offset-path: ray(-120deg closest-corner);
        offset-rotate: reverse 60deg;
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
      "message": "Unknown property “offset-rotate”.",
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
