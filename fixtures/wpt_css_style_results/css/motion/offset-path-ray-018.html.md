# css/motion/offset-path-ray-018.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-018.html"
}
```

## style[0]

```css

  #outer {
    top: 100px;
    left: 100px;
    position: relative;
    width: 200px;
    height: 200px;
    padding: 50px;
    border: 50px solid black;
  }
  #box {
    background-color: green;
    offset-path: ray(0deg sides at 50% 50%) content-box;
    offset-distance: 100%;
    offset-rotate: 0deg;
    offset-anchor: 0% 0%;
    width: 100px;
    height: 100px;
    background-color: green;
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
      "message": "Unknown property “offset-rotate”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “offset-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
