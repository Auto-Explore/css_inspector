# css/css-flexbox/flexbox-writing-mode-slr-rtl.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-writing-mode-slr-rtl.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      width: 40px;
      height: 30px;
      border: 1px solid gray;
      margin-bottom: 5px;
      direction: rtl;
      writing-mode: sideways-lr;
    }
    .flexContainer > * {
      width: 20px;
      height: 15px;
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
