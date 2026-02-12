# css/css-shapes/shape-outside/shape-box/reference/shape-outside-border-box-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/reference/shape-outside-border-box-002-ref.html"
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
    /* Omit shape-outside: border-box; */
    box-sizing: content-box;
    height: 25px;
    width: 25px;
    padding: 25px;
    border: 25px solid lightgreen;
    margin-right: 25px;
    background-color: orange;
  }

  .box {
    display: inline-block;
    width: 25px;
    height: 25px;
    background-color: blue;
  }

  .longbox {
    display: inline-block;
    width: 200px;
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
