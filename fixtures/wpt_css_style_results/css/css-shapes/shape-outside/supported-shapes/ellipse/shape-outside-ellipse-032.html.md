# css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-032.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-032.html"
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
    shape-outside: ellipse(40px 60px at left top);
    clip-path: ellipse(40px 60px at left top);
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 120px;
    background-color: blue;
  }

  .long {
    width: 200px;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
