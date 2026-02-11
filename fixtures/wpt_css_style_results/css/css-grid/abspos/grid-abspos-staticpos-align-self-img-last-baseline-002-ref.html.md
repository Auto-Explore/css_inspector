# css/css-grid/abspos/grid-abspos-staticpos-align-self-img-last-baseline-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-abspos-staticpos-align-self-img-last-baseline-002-ref.html"
}
```

## style[0]

```css

    .container {
      display: block;
      padding: 1px 2px;
      border: 1px solid black;
      background: yellow;
      margin-bottom: 5px;
      margin-right: 5px;
      float: left; /* For testing in "rows" of containers */
    }
    br { clear: both }

    .big > .container {
      height: 40px;
      width: 22px;
    }
    .small > .container {
      height: 2px;
      width: 4px;
      margin-bottom: 20px; /* to reduce overlap between overflowing images */
    }

    .container > * {
      display: block;
    }
    .big   .alignEnd    { margin-top:  24px; }
    .small .alignEnd    { margin-top: -14px; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
