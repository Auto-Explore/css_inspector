# css/motion/offset-path-shape-ellipse-007.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-ellipse-007.html"
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
  top: 100px;
  left: 200px;
  background-color: green;
  position: relative;
  offset-path: ellipse();
  offset-distance: 25%;
  border-radius: 50% 50% 0 0;
  offset-position: 300px 200px;
  width: 100px;
  height: 100px;
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
      "message": "Unknown property “offset-distance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
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
