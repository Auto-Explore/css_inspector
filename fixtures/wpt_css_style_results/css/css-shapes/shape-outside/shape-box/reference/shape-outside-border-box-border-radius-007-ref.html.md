# css/css-shapes/shape-outside/shape-box/reference/shape-outside-border-box-border-radius-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/reference/shape-outside-border-box-border-radius-007-ref.html"
}
```

## style[0]

```css

  .container {
    writing-mode: vertical-lr;
    position: absolute;
    inline-size: 200px;
    line-height: 0;
  }

  .shape {
    float: left;
    /* Omit shape-outside: border-box; */
    border-bottom-right-radius: 60px 40px;
    box-sizing: content-box;
    block-size: 40px;
    inline-size: 40px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin: 20px;
    background-color: orange;
  }

  .box {
    position: absolute;
    inline-size: 60px;
    background-color: blue;
  }

  .longbox {
    position: absolute;
    inline-size: 200px;
    block-size: 20px;
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
