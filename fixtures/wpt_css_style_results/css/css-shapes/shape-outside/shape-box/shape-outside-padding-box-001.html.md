# css/css-shapes/shape-outside/shape-box/shape-outside-padding-box-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/shape-outside-padding-box-001.html"
}
```

## style[0]

```css

  .container {
    width: 175px;
    line-height: 0;
  }

  .shape {
    float: left;
    shape-outside: padding-box;
    box-sizing: content-box;
    height: 25px;
    width: 25px;
    padding: 25px;
    border: 25px solid lightgreen;
    margin: 25px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 25px;
    height: 25px;
    background-color: blue;
  }

  .longbox {
    display: inline-block;
    width: 175px;
    height: 25px;
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
