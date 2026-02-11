# css/motion/offset-path-shape-circle-006.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-circle-006.html"
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
  border-radius: 50%;
}
#box {
  background-color: green;
  position: relative;
  offset-path: circle(100px);
  offset-position: normal;
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
