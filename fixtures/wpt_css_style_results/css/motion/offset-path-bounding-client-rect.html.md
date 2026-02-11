# css/motion/offset-path-bounding-client-rect.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-bounding-client-rect.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  #blue {
    background: radial-gradient(lightblue, blue);
    width: 60px;
    height: 60px;
    offset-path: circle(100px at 150px 150px);
    position: relative;
    left: 100px;
  }
  #purple {
    background: radial-gradient(fuchsia, purple);
    width: 60px;
    height: 60px;
    offset-path: border-box;
    position: relative;
    left: 100px;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-path”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
