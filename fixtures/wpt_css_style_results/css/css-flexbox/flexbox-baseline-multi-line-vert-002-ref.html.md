# css/css-flexbox/flexbox-baseline-multi-line-vert-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-baseline-multi-line-vert-002-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      display: inline-block;
      /* Split testcase's 40px width into 20px of padding-right and 20px of
         width, to set aside space for the testcase's (invisible) second line.
      */
      width: 20px;
      padding-right: 20px;
      height: 40px;
      background: lightblue;
    }
    .flexContainer > * {
      width: 20px;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
