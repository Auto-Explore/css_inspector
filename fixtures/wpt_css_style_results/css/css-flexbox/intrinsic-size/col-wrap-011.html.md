# css/css-flexbox/intrinsic-size/col-wrap-011.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/intrinsic-size/col-wrap-011.html"
}
```

## style[0]

```css

  #reference-overlapped-red {
    position: absolute;
    background-color: red;
    width: 100px;
    height: 100px;
    z-index: -1;
  }

  .item {
    /* Remove min-height so we don't have to think about it. */
    min-height: 0px;
    width: 100px;
    flex: 0 0 auto;
  }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
