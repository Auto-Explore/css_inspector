# css/css-shapes/shape-outside/supported-shapes/inset/shape-outside-inset-017.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/inset/shape-outside-inset-017.html"
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
    shape-outside: inset(20px) border-box;
    clip-path: inset(20px) border-box;
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin: 20px;
    background-color: orange;
  }

  .box {
    display: inline-block;
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
