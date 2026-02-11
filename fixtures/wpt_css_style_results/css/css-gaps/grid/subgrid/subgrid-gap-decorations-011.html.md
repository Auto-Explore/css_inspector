# css/css-gaps/grid/subgrid/subgrid-gap-decorations-011.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/subgrid-gap-decorations-011.html"
}
```

## style[0]

```css

    .grid-container {
      display: grid;
      grid-template-columns: repeat(5, 1fr);
      grid-template-rows: repeat(5, 1fr);
      width: 100px;
      height: 100px;
      background: green;
    }
    .subgrid {
      display: grid;
      grid-template-columns: subgrid;
      grid-template-rows: 1fr 2fr 1fr;

      column-gap: 5px;
      row-gap: 10px;
      background: red;
      grid-column: 2 / -2;
      grid-row: 2 / -2;

      column-rule: green solid 5px;
      row-rule: green solid 10px;
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
