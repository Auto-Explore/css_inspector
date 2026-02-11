# css/css-flexbox/flexbox-baseline-multi-line-horiz-004.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-baseline-multi-line-horiz-004.html"
}
```

## style[0]

```css

    .flexContainer {
      display: inline-flex;
      flex-wrap: wrap-reverse;
      width: 40px;
      height: 100px;
      background: lightgray;

      /* Use "align-content", to test that packing space is considered when
         getting container's baseline from its first FlexLine:*/
      align-content: center;
    }
    .flexContainer > * {
      width: 20px;
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
