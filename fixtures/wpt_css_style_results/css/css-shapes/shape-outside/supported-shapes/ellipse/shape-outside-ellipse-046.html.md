# css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-046.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-046.html"
}
```

## style[0]

```css

  .container {
    writing-mode: vertical-rl;
    inline-size: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    shape-outside: ellipse(farthest-side closest-side at top 40px right 60px) border-box;
    clip-path: ellipse(farthest-side closest-side at top 40px right 60px) border-box;
    box-sizing: content-box;
    block-size: 40px;
    inline-size: 40px;
    padding: 10px 20px;
    border: solid lightgreen;
    border-width: 10px;
    background-color: orange;
  }

  .box {
    display: inline-block;
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
