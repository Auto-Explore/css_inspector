# css/css-shapes/shape-outside/shape-box/reference/shape-outside-content-box-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/reference/shape-outside-content-box-001-ref.html"
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
    /* Omit shape-outside: content-box; */
    box-sizing: content-box;
    height: 25px;
    width: 25px;
    padding-left: 25px;
    border-left: 25px solid lightgreen;
    margin-left: 25px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 50px;
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
