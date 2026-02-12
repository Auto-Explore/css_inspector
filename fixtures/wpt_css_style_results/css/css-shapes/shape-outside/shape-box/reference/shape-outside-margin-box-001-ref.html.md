# css/css-shapes/shape-outside/shape-box/reference/shape-outside-margin-box-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/reference/shape-outside-margin-box-001-ref.html"
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
    /* Omit shape-outside: margin-box; */
    box-sizing: content-box;
    height: 25px;
    width: 25px;
    padding: 25px;
    border: 25px solid lightgreen;
    margin: 25px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 25px;
    height: 25px;
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
