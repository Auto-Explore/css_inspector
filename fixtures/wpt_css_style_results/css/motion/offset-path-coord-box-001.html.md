# css/motion/offset-path-coord-box-001.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-coord-box-001.html"
}
```

## style[0]

```css

#outer {
  top: 100px;
  left: 100px;
  position: relative;
  border-radius: 50%;
  border: 10px solid black;
  width: 400px;
  height: 400px;
}
#box {
  background-color: green;
  top: 100px;
  left: 100px;
  position: relative;
  offset-path: border-box;
  offset-distance: 25%;
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
      "message": "Unknown property “offset-distance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
