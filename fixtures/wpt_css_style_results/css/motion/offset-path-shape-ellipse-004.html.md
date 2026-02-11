# css/motion/offset-path-shape-ellipse-004.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-ellipse-004.html"
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
  offset-path: ellipse(10% 10% at bottom 25% right 25%);
  offset-anchor: 100% 100%;
  offset-distance: 75%;
  width: 100px;
  height: 100px;
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
      "message": "Unknown property “offset-anchor”.",
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
