# css/css-shapes/shape-outside/supported-shapes/circle/reference/shape-outside-circle-036-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/circle/reference/shape-outside-circle-036-ref.html"
}
```

## style[0]

```css

  .container {
    position: absolute;
    width: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    /* Omit shape-outside */
    clip-path: circle();
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
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
