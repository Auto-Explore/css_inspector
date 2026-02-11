# css/css-break/relpos-inline-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/relpos-inline-ref.html"
}
```

## style[0]

```css

  span {
      position: relative;
      left: -100px;
      top: 100px;
      border: 1px solid;
      background:hotpink;
  }
  .fakecol {
      float: left;
      width: 150px;
      height: 100%;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
