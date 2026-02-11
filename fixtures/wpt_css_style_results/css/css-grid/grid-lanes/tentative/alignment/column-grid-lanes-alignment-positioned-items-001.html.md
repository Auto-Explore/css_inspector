# css/css-grid/grid-lanes/tentative/alignment/column-grid-lanes-alignment-positioned-items-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/column-grid-lanes-alignment-positioned-items-001.html"
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
      grid-template-columns: 100px 150px;
      width: 300px;
      height: 200px;
      background: grey;
      gap: 10px;
      padding: 10px;
      justify-items: start;
    }

    .grid-lanes > div {
      position: absolute;
    }

    .grid-lanes > :nth-child(1) {
      grid-column: 1 / 2;
      align-self: start;
      background: green;
    }

    .grid-lanes > :nth-child(2) {
      grid-column: 2 / 3;
      align-self: start;
      background: blue;
    }

    .grid-lanes > :nth-child(3) {
      grid-column: 1 / 2;
      align-self: end;
      background: yellow;
    }

    .grid-lanes > :nth-child(4) {
      grid-column: 2 / 3;
      align-self: end;
      background: red;
    }
  
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
