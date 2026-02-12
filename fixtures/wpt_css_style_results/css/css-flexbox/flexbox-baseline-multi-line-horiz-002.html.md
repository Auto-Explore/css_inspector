# css/css-flexbox/flexbox-baseline-multi-line-horiz-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-baseline-multi-line-horiz-002.html"
}
```

## style[0]

```css

    .flexContainer {
      display: inline-flex;
      flex-wrap: wrap-reverse;
      width: 40px;
      height: 40px;
      background: lightblue;
    }
    .flexContainer > * {
      width: 20px;
    }

    /* We'll make the second flex line not paint anything, so that the
       reference case doesn't need to bother matching it. */
    .flexContainer > *:nth-child(1),
    .flexContainer > *:nth-child(2) {
      visibility: hidden;
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
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
