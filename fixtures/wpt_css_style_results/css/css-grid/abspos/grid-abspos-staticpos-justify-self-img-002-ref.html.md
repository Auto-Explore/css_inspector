# css/css-grid/abspos/grid-abspos-staticpos-justify-self-img-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-abspos-staticpos-justify-self-img-002-ref.html"
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
      margin-right: 9px;
      float: left; /* For testing in "rows" of containers */
    }
    br { clear: both }

    .big > .container {
      width: 40px;
      height: 22px;
    }
    .small > .container {
      width: 2px;
      height: 4px;
      margin-bottom: 20px; /* to reduce overlap between overflowing images */
    }

    .container > * {
      display: block;
    }
    .big   .alignStart  { margin-left:  0px; }
    .big   .alignCenter { margin-left: 16px; }
    .big   .alignEnd    { margin-left: 32px; }
    .small .alignStart  { margin-left:  0px; }
    .small .alignCenter { margin-left: -3px; }
    .small .alignEnd    { margin-left: -6px; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
