# css/css-shapes/shape-outside/supported-shapes/polygon/shape-outside-polygon-019.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/polygon/shape-outside-polygon-019.html"
}
```

## style[0]

```css

  .container {
    writing-mode: horizontal-tb;
    direction: rtl;
    width: 200px;
    line-height: 0;
  }

  .shape {
    float: right;
    shape-outside: polygon(60px 20px, 100px 60px, 20px 60px, 60px 100px) border-box;
    clip-path: polygon(60px 20px, 100px 60px, 20px 60px, 60px 100px) border-box;
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px;
    margin: 20px;
    border: 20px solid lightgreen;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 80px;
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
