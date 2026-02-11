# css/motion/offset-path-shape-inset-002.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-inset-002.html"
}
```

## style[0]

```css

#outer {
  top: 100px;
  left: 100px;
  position: relative;
  width: 400px;
  height: 400px;
}
#box {
  background-color: green;
  position: relative;
  offset-path: inset(50px round 50%);
  offset-anchor: 0 0;
  offset-distance: 12.5%;
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
      "message": "Unknown property “offset-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-distance”.",
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
