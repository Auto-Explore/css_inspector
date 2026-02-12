# css/css-shapes/shape-outside/shape-box/reference/shape-outside-content-box-border-radius-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/reference/shape-outside-content-box-border-radius-001-ref.html"
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
    /* Omit shape-outside: content-box; */
    border-radius: 50%;
    box-sizing: content-box;
    height: 120px;
    width: 120px;
    padding: 10px;
    border: 5px solid lightgreen;
    margin: 5px;
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
