# css/motion/offset-path-string-003.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-string-003.html"
}
```

## style[0]

```css

      #target {
        position: absolute;
        left: 50px;
        top: 80px;
        width: 200px;
        height: 300px;
        background-color: lime;
        transform-origin: 0px 0px;
        offset-path: path('M 50 40');
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
