# css/css-grid/grid-lanes/tentative/alignment/row-grid-lanes-alignment-positioned-items-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/row-grid-lanes-alignment-positioned-items-001.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:10px/1 monospace; padding:0; margin:0;
    }

    .grid-lanes {
      position: relative;
      display: grid-lanes;
      grid-lanes-direction: row;
      grid-template-rows: 100px 150px;
      width: 200px;
      height: 300px;
      background: grey;
      gap: 10px;
      padding: 10px;
      align-items: start;
    }

    .grid-lanes > div {
      position: absolute;
    }

    .grid-lanes > :nth-child(1) {
      grid-row: 1 / 2;
      justify-self: start;
      background: green;
    }

    .grid-lanes > :nth-child(2) {
      grid-row: 2 / 3;
      justify-self: start;
      background: blue;
    }

    .grid-lanes > :nth-child(3) {
      grid-row: 1 / 2;
      justify-self: end;
      background: yellow;
    }

    .grid-lanes > :nth-child(4) {
      grid-row: 2 / 3;
      justify-self: end;
      background: red;
    }
  
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
