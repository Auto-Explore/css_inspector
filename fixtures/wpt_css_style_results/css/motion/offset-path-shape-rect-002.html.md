# css/motion/offset-path-shape-rect-002.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-rect-002.html"
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
  offset-path: rect(5px 95% 95% 5px round 30%);
  offset-distance: 45%;
  offset-anchor: 10% 10%;
  border-radius: 50% 50% 0 0;
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
      "message": "Unknown property “offset-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
