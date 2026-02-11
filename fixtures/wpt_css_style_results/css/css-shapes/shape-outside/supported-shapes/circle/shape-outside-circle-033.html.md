# css/css-shapes/shape-outside/supported-shapes/circle/shape-outside-circle-033.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/circle/shape-outside-circle-033.html"
}
```

## style[0]

```css

  .container {
    width: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    shape-outside: circle(50% at right bottom);
    clip-path: circle(50% at right bottom);
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 60px;
    background-color: blue;
  }

  .long {
    width: 200px;
  }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
