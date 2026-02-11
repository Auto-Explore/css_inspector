# css/css-shapes/shape-outside/shape-box/shape-outside-border-box-border-radius-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/shape-outside-border-box-border-radius-001.html"
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
    shape-outside: border-box;
    border-radius: 50%;
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin: 20px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 60px;
    background-color: blue;
  }

  .longbox {
    display: inline-block;
    width: 200px;
    height: 20px;
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
