# css/css-shapes/shape-outside/supported-shapes/inset/shape-outside-inset-027.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/inset/shape-outside-inset-027.html"
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
    shape-outside: inset(10px round 60px 0/ 40px 0) border-box;
    clip-path: inset(10px round 60px 0/ 40px 0) border-box;
    box-sizing: content-box;
    block-size: 40px;
    inline-size: 40px;
    padding: 10px 20px;
    border: solid lightgreen;
    border-width: 20px 30px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    inline-size: 80px;
    background-color: blue;
  }

  .long {
    inline-size: 200px;
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
