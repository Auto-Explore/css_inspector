# css/css-flexbox/abspos/flex-abspos-staticpos-justify-content-vertWM-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/abspos/flex-abspos-staticpos-justify-content-vertWM-002.html"
}
```

## style[0]

```css

    .container {
      display: flex;
      position: relative;
      flex-flow: column;
      writing-mode: vertical-rl;
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
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
