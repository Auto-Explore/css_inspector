# css/css-grid/abspos/grid-abspos-staticpos-align-self-safe-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-abspos-staticpos-align-self-safe-001.html"
}
```

## style[0]

```css

    .container {
      display: grid;
      padding: 2px 1px;
      border: 1px solid black;
      background: yellow;
      margin-bottom: 5px;
      margin-right: 5px;
      float: left; /* For testing in "rows" of containers */
      position: relative;
      width: 30px;
      height: 25px;
      grid: 3px 14px 3px / 2px 20px 2px;
    }
    br { clear: both }
    .container > * {
      position: absolute;
      grid-area: 2 / 2 / 3 / 3;
      background: teal;
      width: 21px;
      height: 21px;
      justify-self: safe end;
      align-self: safe center;
    }
    .vertRL {
      writing-mode: vertical-rl;
    }
    .relPos {
      position: relative;
    }
    .relPos > * {
      height: 35px;
      width: 33px;
    }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-area”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
