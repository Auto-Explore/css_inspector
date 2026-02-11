# css/motion/offset-path-shape-ellipse-002.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-ellipse-002.html"
}
```

## style[0]

```css

#outer {
  top: 100px;
  left: 100px;
  position: relative;
  width: 600px;
  height: 400px;
}
#box {
  background-color: green;
  position: relative;
  offset-path: ellipse();
  offset-position: auto;
  width: 100px;
  height: 100px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “offset-path”.",
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
