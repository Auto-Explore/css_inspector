# css/css-shapes/shape-outside/shape-box/shape-outside-margin-box-border-radius-007.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-box/shape-outside-margin-box-border-radius-007.html"
}
```

## style[0]

```css

  .bfc {
    width: 200px;
    line-height: 0;
    direction: ltr;
  }

  .container {
    direction: rtl;
    unicode-bidi: bidi-override;
  }

  .shape {
    float: left;
    shape-outside: margin-box;
    border-top-right-radius: 50%;
    box-sizing: content-box;
    height: 20px;
    width: 20px;
    padding: 20px;
    border: 20px solid lightgreen;
    margin: 10px;
    background-color: orange;
  }

  .box {
    display: inline-block;
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
