# css/css-gaps/grid/subgrid/subgrid-gap-decorations-002.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/subgrid-gap-decorations-002.html"
}
```

## style[0]

```css

  .grid-container {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(3, 1fr);
    gap: 10px;
    width: 100px;
    height: 100px;
  }
  .subgrid {
    display: grid;
    grid-template-columns: subgrid;
    grid-template-rows: repeat(2, 1fr);

    grid-column: 1 / -1;
    grid-row: 1 / -1;
    column-rule: green solid 10px;
    row-gap: 20px;
    row-rule: green solid 20px;

    background: red;
  }
  .subgrid-item {
    background: green;
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
