# css/css-grid/abspos/grid-abspos-staticpos-justify-self-last-baseline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-abspos-staticpos-justify-self-last-baseline-001.html"
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
      justify-items: center; /* To exercise 'justify-self: auto' on children */
      position: relative;
    }
    br { clear: both }

    .big > .container {
      width: 30px;
      height: 22px;
      grid: 3px 14px 3px / 2px 20px 2px;
    }
    .small > .container {
      grid: 3px 2px 3px / 0px 2px 0px;
      width: 2px;
      height: 4px;
      margin-right: 10px; /* To avoid overlap between overflowing kids */
    }

    .container > * {
      position: absolute;
      grid-area: 2 / 2 / 3 / 3;
      background: teal;
      width: 6px;
      height: 8px;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
