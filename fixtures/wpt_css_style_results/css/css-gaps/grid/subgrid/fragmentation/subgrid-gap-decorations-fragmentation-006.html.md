# css/css-gaps/grid/subgrid/fragmentation/subgrid-gap-decorations-fragmentation-006.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/fragmentation/subgrid-gap-decorations-fragmentation-006.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multicol {
    position: relative;
    columns: 2;
    column-fill: auto;
    width: 280px;
    height: 80px;
    background: yellow;
    column-gap: 0px;
  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    grid-template-rows: repeat(5, 1fr);
    background: lightgray;
    gap: 10px;
    height: 140px;
    column-rule: skyblue solid 10px;
    row-rule: cornflowerblue solid 10px;
    column-rule-break: none;
    row-rule-break: none;
  }
  .subgrid {
    display: grid;
    grid-template-columns: subgrid;
    grid-template-rows: subgrid;
    background: orange;
    grid-column: 2 / -2;
    grid-row: 2 / -2;
    column-rule: white solid 10px;
    row-rule: blue solid 10px;
  }
  .item {
    background: purple;
  }
```

```json
{
  "errors": 10,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-break”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
    }
  ],
  "warnings": 0
}
```
