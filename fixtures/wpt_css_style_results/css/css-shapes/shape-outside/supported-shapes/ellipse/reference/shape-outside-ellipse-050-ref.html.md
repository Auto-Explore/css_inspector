# css/css-shapes/shape-outside/supported-shapes/ellipse/reference/shape-outside-ellipse-050-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/reference/shape-outside-ellipse-050-ref.html"
}
```

## style[0]

```css

  .container {
    writing-mode: sideways-lr;
    position: absolute;
    inline-size: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    /* Omit shape-outside */
    clip-path: ellipse(farthest-side closest-side at top 40px left 60px) border-box;
    box-sizing: content-box;
    block-size: 40px;
    inline-size: 40px;
    padding: 10px 20px;
    border: solid lightgreen;
    border-width: 10px;
    margin-block-end: 20px;
    background-color: orange;
  }

  .box {
    position: absolute;
    inline-size: 80px;
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
