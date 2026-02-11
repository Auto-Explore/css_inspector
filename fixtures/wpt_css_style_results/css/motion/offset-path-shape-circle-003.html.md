# css/motion/offset-path-shape-circle-003.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-circle-003.html"
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
  offset-path: circle(farthest-side at top);
  offset-distance: 18%;
  border-radius: 50% 50% 0 0;
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
