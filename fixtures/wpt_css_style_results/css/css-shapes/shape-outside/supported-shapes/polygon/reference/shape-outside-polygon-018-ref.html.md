# css/css-shapes/shape-outside/supported-shapes/polygon/reference/shape-outside-polygon-018-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/polygon/reference/shape-outside-polygon-018-ref.html"
}
```

## style[0]

```css

  .container {
    writing-mode: horizontal-tb;
    position: absolute;
    width: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    /* Omit shape-outside */
    clip-path: polygon(60px 20px, 100px 60px, 20px 60px, 60px 100px) border-box;
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin: 20px;
    background-color: orange;
  }

  .box {
    position: absolute;
    width: 80px;
    background-color: blue;
  }

  .long {
    width: 200px;
  }
  
```

```json
{
  "errors": 2,
  "messages": [
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
