# css/css-flexbox/flexbox-baseline-align-self-baseline-vert-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-baseline-align-self-baseline-vert-001.html"
}
```

## style[0]

```css

    body {
      margin: 0;
      font: 20px Ahem;
      line-height: 20px;
      /* Baseline is 0.8em = 16px from top */
    }
    .flexContainer {
      display: inline-flex;
      flex-direction: column;
      background: lightblue;
      align-items: baseline;
    }
    .hugeAndUnaligned {
      font-size: 35px;
      line-height: 35px;
      /* This one flex item won't be baseline-aligned, so it won't impact
         the flex container's positioning */
      align-self: stretch;
    }
    .smallFont {
      font-size: 10px;
      line-height: 10px;
      /* Baseline is 0.8em = 8px from top */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
