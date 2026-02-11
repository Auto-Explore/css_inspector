# css/css-flexbox/flexbox-baseline-multi-line-horiz-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-baseline-multi-line-horiz-004-ref.html"
}
```

## style[0]

```css

    /* We use an outer vertical flex container, wrapping two single-line
       flex containers, to match the testcase's multi-line flex container. */
    .outerFlexContainer {
      height: 100px;
      background: lightgray;
      display: inline-flex;
      flex-direction: column-reverse;
      justify-content: center; /* to mimic testcase's "align-content:center" */
    }
    .flexContainer {
      display: flex;
      width: 40px;
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
