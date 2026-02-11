# css/css-shapes/shape-outside/shape-box/reference/shape-outside-margin-box-border-radius-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/reference/shape-outside-margin-box-border-radius-007-ref.html"
}
```

## style[0]

```css

  .bfc {
    position: absolute;
    width: 200px;
    line-height: 0;
    direction: ltr;
  }

  .container {
    direction: rtl;
    unicode-bidi: bidi-override;
  }

  .shape {
    float: left;
    /* Omit shape-outside: margin-box; */
    border-top-right-radius: 50%;
    box-sizing: content-box;
    height: 20px;
    width: 20px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin: 10px;
    background-color: orange;
  }

  .box {
    position: absolute;
    width: 60px;
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
