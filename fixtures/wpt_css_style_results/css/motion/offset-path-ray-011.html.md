# css/motion/offset-path-ray-011.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-011.html"
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
    offset-path: ray(90deg);
    offset-distance: 10px;
    offset-position: normal;
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
      "message": "Unknown property “offset-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
