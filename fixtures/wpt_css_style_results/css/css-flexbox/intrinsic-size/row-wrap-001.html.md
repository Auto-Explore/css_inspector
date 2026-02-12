# css/css-flexbox/intrinsic-size/row-wrap-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/intrinsic-size/row-wrap-001.html"
}
```

## style[0]

```css

  .zero-width {
    width: 0px;
    height: 100px;
    margin-bottom: 20px;
  }

  .floating-flexbox {
    display: flex;
    flex-wrap: wrap;
    outline: 5px solid blue;
    height: 100px;
    float: left;
  }

  .floating-flexbox>div:nth-child(1) {
    background: yellow;
  }

  .floating-flexbox>div:nth-child(2) {
    background: orange;
  }

  .floating-flexbox>div>div {
    width: 100px;
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
