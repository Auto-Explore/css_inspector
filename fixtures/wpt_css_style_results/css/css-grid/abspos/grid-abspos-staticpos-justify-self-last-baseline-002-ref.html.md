# css/css-grid/abspos/grid-abspos-staticpos-justify-self-last-baseline-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-abspos-staticpos-justify-self-last-baseline-002-ref.html"
}
```

## style[0]

```css

    .container {
      display: block;
      padding: 2px 1px;
      border: 1px solid black;
      background: yellow;
      margin-bottom: 5px;
      margin-right: 5px;
      float: left; /* For testing in "rows" of containers */
    }
    br { clear: both }

    .big > .container {
      width: 30px;
      height: 22px;
    }
    .small > .container {
      width: 2px;
      height: 4px;
      margin-right: 10px; /* To avoid overlap between overflowing kids */
    }

    .container > * {
      background: teal;
      width: 6px;
      height: 8px;
    }
    .big   .alignEnd    { margin-left: 24px; }
    .small .alignEnd    { margin-left: -4px; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
