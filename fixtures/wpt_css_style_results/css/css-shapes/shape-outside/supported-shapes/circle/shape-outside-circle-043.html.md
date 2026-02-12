# css/css-shapes/shape-outside/supported-shapes/circle/shape-outside-circle-043.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/circle/shape-outside-circle-043.html"
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
    shape-outside: circle(closest-side at center) border-box;
    clip-path: circle(closest-side at center) border-box;
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin-left: 10px; /* Not affect layout of the boxes */
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 60px;
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
