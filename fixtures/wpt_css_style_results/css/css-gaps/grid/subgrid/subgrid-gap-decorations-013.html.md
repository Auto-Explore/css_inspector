# css/css-gaps/grid/subgrid/subgrid-gap-decorations-013.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/subgrid-gap-decorations-013.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }

  .grid-container {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    grid-template-rows: repeat(5, 1fr);

    gap: 10px;
    width: 140px;
    height: 140px;

    column-rule: skyblue solid 10px;
    row-rule: cornflowerblue solid 10px;
  }

  .subgrid {
    display: grid;
    grid-template-columns: subgrid;
    grid-template-rows: subgrid;

    background: red;
    grid-column: 2 / -2;
    grid-row: 2 / -2;

    column-rule: green solid 10px;
    row-rule: blue solid 10px;
  }

  .item {
    background: purple;
    opacity: 0.5;
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule”.",
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
