# css/css-shapes/shape-outside/supported-shapes/circle/shape-outside-circle-052.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/circle/shape-outside-circle-052.html"
}
```

## style[0]

```css

  .container {
    writing-mode: sideways-lr;
    inline-size: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    shape-outside: circle(50% at right 40px bottom 40px) border-box;
    clip-path: circle(50% at right 40px bottom 40px) border-box;
    box-sizing: content-box;
    block-size: 40px;
    inline-size: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin-inline-start: 20px;
    margin-inline-end: 20px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    inline-size: 60px;
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
