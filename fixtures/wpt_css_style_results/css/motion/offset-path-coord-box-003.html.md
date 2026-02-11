# css/motion/offset-path-coord-box-003.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-coord-box-003.html"
}
```

## style[0]

```css

#outer {
  top: 100px;
  left: 100px;
  position: relative;
  border: 10px solid black;
  padding: 10px;
  width: 600px;
  height: 400px;
}
#box {
  background-color: green;
  top: 100px;
  left: 100px;
  position: relative;
  offset-path: content-box;
  offset-distance: 15%;
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
