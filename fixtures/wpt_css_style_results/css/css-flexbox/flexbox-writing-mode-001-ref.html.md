# css/css-flexbox/flexbox-writing-mode-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-writing-mode-001-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      width: 40px;
      height: 30px;
      border: 1px solid gray;
      margin-bottom: 5px;
      /* Testcase has direction: ltr; */
      /* Testcase has writing-mode: horizontal-tb; */
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
