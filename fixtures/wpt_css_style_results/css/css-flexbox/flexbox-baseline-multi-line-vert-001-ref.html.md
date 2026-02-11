# css/css-flexbox/flexbox-baseline-multi-line-vert-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-baseline-multi-line-vert-001-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      display: inline-block;
      width: 40px;
      height: 40px;
      background: lightblue;
    }
    .flexContainer > * {
      height: 20px;
      display: inline-block;
    }

    .smallFont {
      font-size: 8px;
      line-height: 8px;
    }
    .medFont {
      font-size: 12px;
      line-height: 12px;
    }
    .bigFont {
      font-size: 16px;
      line-height: 16px;
    }
    .unaligned { float: left }
  
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
