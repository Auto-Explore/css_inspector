# css/css-flexbox/abspos/flex-abspos-staticpos-align-content-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/abspos/flex-abspos-staticpos-align-content-001.html"
}
```

## style[0]

```css

    .container {
      display: flex;
      position: relative;
      flex-flow: row wrap;
      padding: 1px 2px;
      border: 1px solid black;
      background: yellow;
      margin-bottom: 5px;
      margin-right: 5px;
      float: left; /* For testing in "rows" of containers */
    }
    br { clear: both }

    .big > .container {
      height: 10px;
      width: 16px;
    }
    .small > .container {
      height: 2px;
      width: 4px;
    }

    .container > * {
      position: absolute;
      background: teal;
      height: 6px;
      width: 8px;
      /* This "align-self" is actually the only alignment that matters here.
         The flex containers' various "align-content" values have no impact on
         the positioning of absolutely-positioned flex children. */
      align-self: center;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
