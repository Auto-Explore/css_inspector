# css/css-shapes/shape-outside/supported-shapes/inset/reference/shape-outside-inset-020-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/inset/reference/shape-outside-inset-020-ref.html"
}
```

## style[0]

```css

  .container {
    writing-mode: horizontal-tb;
    position: absolute;
    inline-size: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    /* Omit shape-outside */
    clip-path: inset(10px round 0 40px/ 0 60px) border-box;
    box-sizing: content-box;
    block-size: 40px;
    inline-size: 40px;
    padding: 20px 10px;
    border: solid lightgreen;
    border-width: 30px 20px;
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
