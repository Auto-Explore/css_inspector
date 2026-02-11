# css/css-shapes/shape-outside/supported-shapes/inset/reference/shape-outside-inset-024-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/inset/reference/shape-outside-inset-024-ref.html"
}
```

## style[0]

```css

  .container {
    writing-mode: vertical-lr;
    position: absolute;
    inline-size: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    /* Omit shape-outside */
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
    position: absolute;
    inline-size: 80px;
    background-color: blue;
  }

  .long {
    inline-size: 200px;
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
