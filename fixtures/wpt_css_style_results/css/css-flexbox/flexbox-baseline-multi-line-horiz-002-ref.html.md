# css/css-flexbox/flexbox-baseline-multi-line-horiz-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-baseline-multi-line-horiz-002-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      display: inline-block;
      width: 40px;
      /* Split testcase's 40px height into 20px of padding-bottom and 20px of
         height, to set aside space for the testcase's (invisible) second line. */
      height: 20px;
      padding-bottom: 20px;
      background: lightblue;
    }
    .flexContainer > * {
      width: 20px;
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
    .unaligned { vertical-align: top }
  
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
