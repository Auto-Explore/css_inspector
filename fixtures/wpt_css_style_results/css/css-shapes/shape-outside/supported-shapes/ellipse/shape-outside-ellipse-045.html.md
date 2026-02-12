# css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-045.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-045.html"
}
```

## style[0]

```css

  .container {
    direction: rtl;
    width: 200px;
    line-height: 0;
  }

  .shape {
    float: right;
    shape-outside: ellipse(100% 100%);
    clip-path: ellipse(100% 100%); /* Rendered as a rectangle */
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px 10px;
    border: solid lightgreen;
    border-width: 20px 10px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 80px;
    background-color: blue;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
