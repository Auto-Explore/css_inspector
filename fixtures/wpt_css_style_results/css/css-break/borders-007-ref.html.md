# css/css-break/borders-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/borders-007-ref.html"
}
```

## style[0]

```css

  .box {
      height: 80px;
      margin-bottom: 10px;
      border: 10px solid hotpink;
      background: yellow;
  }
  .skip-start {
      border-right: none;
  }
  .skip-end {
      border-left: none;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
