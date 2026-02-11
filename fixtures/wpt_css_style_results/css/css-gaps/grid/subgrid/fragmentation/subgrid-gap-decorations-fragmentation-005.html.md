# css/css-gaps/grid/subgrid/fragmentation/subgrid-gap-decorations-fragmentation-005.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/fragmentation/subgrid-gap-decorations-fragmentation-005.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multi-col {
    height: 100px;
    width: 320px;
    columns: 3;
    column-fill: auto;
    column-gap: 10px;
    background: lightgray;
  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(4, 90px);
    column-gap: 10px;
    row-gap: 20px;
    row-rule: solid 10px red;
    column-rule: solid 6px blue;
  }
  .subgrid {
    display: grid;
    grid-column: 1 / -1;
    grid-row: 1 / -1;
    grid-template-columns: subgrid;
    grid-template-rows: subgrid;
    column-rule: blue solid 6px;
    row-rule: red solid 5px;
  }
  .subgrid > div {
    background-color: skyblue;
  }
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
