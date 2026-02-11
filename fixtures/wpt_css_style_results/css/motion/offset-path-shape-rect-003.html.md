# css/motion/offset-path-shape-rect-003.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-rect-003.html"
}
```

## style[0]

```css

#outer {
  top: 100px;
  left: 100px;
  position: relative;
  width: 200px;
  height: 100px;
  padding: 50px;
  border: 50px solid black;
}
#box {
  background-color: green;
  position: relative;
  offset-path: rect(auto auto 50% 10px) padding-box;
  offset-distance: 40%;
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
