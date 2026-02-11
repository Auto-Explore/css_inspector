# css/css-gaps/grid/subgrid/subgrid-gap-decorations-007.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/subgrid-gap-decorations-007.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(3, 100px);
    grid-template-rows: repeat(3, 100px);
    gap: 10px;
  }
  .subgrid {
    display: grid;
    grid-column: 1 / -1;
    grid-row: 1 / -1;
    grid-template-columns: subgrid;
    grid-template-rows: subgrid;

    column-rule: blue solid 5px;
    row-rule: red solid 5px;

    column-rule-inset: 0px;
    column-rule-break: intersection;
  }
  .item {
    background: gray;
    opacity: 0.5;
  }
```

```json
{
  "errors": 6,
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
    },
    {
      "message": "Unknown property “column-rule-inset”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column-rule-break”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
