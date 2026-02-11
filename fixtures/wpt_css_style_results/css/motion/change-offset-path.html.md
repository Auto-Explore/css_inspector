# css/motion/change-offset-path.html

```json
{
  "format_version": 3,
  "file": "css/motion/change-offset-path.html"
}
```

## style[0]

```css

#target {
  will-change: transform;
  transform: translate(50px, 50px);
  offset-path: path('M 0 0 L 200 200 Z');
  width: 100px;
  height: 100px;
  background: green;
}
#container {
  width: 100px;
  height: 100px;
  background: red;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
