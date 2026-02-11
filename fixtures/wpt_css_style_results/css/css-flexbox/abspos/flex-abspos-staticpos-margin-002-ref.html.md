# css/css-flexbox/abspos/flex-abspos-staticpos-margin-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/abspos/flex-abspos-staticpos-margin-002-ref.html"
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
      margin-right: 10px;
      float: left; /* For testing in "rows" of containers */
    }
    br { clear: both }

    .big > .container {
      height: 14px;
      width: 20px;
    }
    .small > .container {
      height: 2px;
      width: 4px;
    }

    .container > * {
      background: teal;
      height: 6px;
      width: 8px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
