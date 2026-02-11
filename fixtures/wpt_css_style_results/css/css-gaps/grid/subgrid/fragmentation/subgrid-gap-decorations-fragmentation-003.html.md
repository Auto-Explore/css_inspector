# css/css-gaps/grid/subgrid/fragmentation/subgrid-gap-decorations-fragmentation-003.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/fragmentation/subgrid-gap-decorations-fragmentation-003.html"
}
```

## style[0]

```css

  .multi-col {
    position: relative;
    columns: 3;
    column-fill: auto;
    width: 100px;
    height: 100px;
    background: green;
    column-gap: 0px;
  }
  .grid-container {
    display: grid;
    grid-template-rows: repeat(3, 90px);
    row-gap: 15px;
    column-gap: 10px;
  }
  .subgrid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: subgrid;
    grid-column: 1 / -1;
    grid-row: 1 / -1;
    column-rule: green solid 10px;
    column-gap: 10px;
    row-rule: green solid 20px;
    background: red;
  }
  .subgrid-item {
    background: green;
  }
  .abspos {
    background: green;
    position: absolute;
    height: 10px;
    top: 90px;
  }
```

```json
{
  "errors": 4,
  "messages": [
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
