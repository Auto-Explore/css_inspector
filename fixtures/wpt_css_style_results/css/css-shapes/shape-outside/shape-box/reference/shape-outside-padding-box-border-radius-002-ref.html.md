# css/css-shapes/shape-outside/shape-box/reference/shape-outside-padding-box-border-radius-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/reference/shape-outside-padding-box-border-radius-002-ref.html"
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
    /* Omit shape-outside: border-box; */
    border-radius: 50%;
    box-sizing: content-box;
    height: 40px;
    width: 40px;
    padding: 40px;
    border: 10px solid lightgreen;
    margin: 10px;
    background-color: orange;
  }

  .box {
    position: absolute;
    width: 60px;
    background-color: blue;
  }

  .longbox {
    position: absolute;
    width: 200px;
    height: 20px;
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
