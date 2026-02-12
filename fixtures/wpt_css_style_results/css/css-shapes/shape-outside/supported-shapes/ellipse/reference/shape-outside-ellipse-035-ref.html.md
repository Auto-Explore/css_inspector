# css/css-shapes/shape-outside/supported-shapes/ellipse/reference/shape-outside-ellipse-035-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/reference/shape-outside-ellipse-035-ref.html"
}
```

## style[0]

```css

  .container {
    direction: rtl;
    position: absolute;
    width: 200px;
    line-height: 0;
  }

  .shape {
    float: right;
    /* Omit shape-outside */
    clip-path: ellipse(40px 60px at left bottom);
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    background-color: orange;
  }

  .box {
    position: absolute;
    width: 60px;
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
