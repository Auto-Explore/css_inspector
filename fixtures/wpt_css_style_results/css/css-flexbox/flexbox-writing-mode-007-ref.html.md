# css/css-flexbox/flexbox-writing-mode-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-writing-mode-007-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      width: 40px;
      height: 30px;
      border: 1px solid gray;
      margin-bottom: 5px;
    }
    .flexContainer > * {
      width: 20px;
      height: 15px;
      float: left;
    }
    .item1 {
      /* Note: flex items are ordered as "CMYK": cyan, magenta, yellow, black */
      background: cyan;
    }
    .item2 {
      background: magenta;
    }
    .item3 {
      background: yellow;
    }
    .item4 {
      background: black;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
