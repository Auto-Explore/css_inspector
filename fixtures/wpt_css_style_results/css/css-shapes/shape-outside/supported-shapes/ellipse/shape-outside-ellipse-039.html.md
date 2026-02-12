# css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-039.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-039.html"
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
    shape-outside: ellipse(closest-side farthest-side at right 40px top 60px) border-box;
    clip-path: ellipse(closest-side farthest-side at right 40px top 60px) border-box;
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px 10px;
    border: solid lightgreen;
    border-width: 10px;
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
