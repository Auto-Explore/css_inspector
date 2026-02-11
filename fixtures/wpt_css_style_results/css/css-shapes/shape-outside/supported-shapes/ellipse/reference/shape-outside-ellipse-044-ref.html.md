# css/css-shapes/shape-outside/supported-shapes/ellipse/reference/shape-outside-ellipse-044-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/reference/shape-outside-ellipse-044-ref.html"
}
```

## style[0]

```css

  .container {
    position: absolute;
    width: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    /* Omit shape-outside */
    clip-path: ellipse(100% 100%);
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px 10px;
    border: solid lightgreen;
    border-width: 20px 10px;
    background-color: orange;
  }

  .box {
    position: absolute;
    width: 80px;
    background-color: blue;
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
