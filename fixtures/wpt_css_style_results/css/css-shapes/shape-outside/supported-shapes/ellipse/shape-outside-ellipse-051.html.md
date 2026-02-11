# css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-051.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-051.html"
}
```

## style[0]

```css

  .container {
    writing-mode: sideways-lr;
    direction: rtl;
    inline-size: 200px;
    line-height: 0;
  }

  .shape {
    float: right;
    shape-outside: ellipse(farthest-side closest-side at top 40px left 60px) border-box;
    clip-path: ellipse(farthest-side closest-side at top 40px left 60px) border-box;
    box-sizing: content-box;
    block-size: 40px;
    inline-size: 40px;
    padding: 10px 20px;
    border: solid lightgreen;
    border-width: 10px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    inline-size: 80px;
    background-color: blue;
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
