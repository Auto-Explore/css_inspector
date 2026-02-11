# css/css-shapes/shape-outside/shape-box/shape-outside-border-box-border-radius-007.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/shape-outside-border-box-border-radius-007.html"
}
```

## style[0]

```css

  .container {
    writing-mode: vertical-lr;
    inline-size: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    shape-outside: border-box;
    border-bottom-right-radius: 60px 40px;
    box-sizing: content-box;
    block-size: 40px;
    inline-size: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin: 20px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    inline-size: 60px;
    background-color: blue;
  }

  .longbox {
    display: inline-block;
    inline-size: 200px;
    block-size: 20px;
    background-color: blue;
  }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-right-radius”.",
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
