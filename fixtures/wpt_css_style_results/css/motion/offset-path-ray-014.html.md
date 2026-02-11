# css/motion/offset-path-ray-014.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-014.html"
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
    top: 100px;
    left: 100px;
    offset-path: ray(0deg closest-side);
    offset-distance: 100%;
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
