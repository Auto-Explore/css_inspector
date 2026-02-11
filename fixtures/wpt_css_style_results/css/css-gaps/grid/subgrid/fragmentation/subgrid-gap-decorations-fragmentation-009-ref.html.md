# css/css-gaps/grid/subgrid/fragmentation/subgrid-gap-decorations-fragmentation-009-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/fragmentation/subgrid-gap-decorations-fragmentation-009-ref.html"
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
    grid-template-rows: 20px 10px 20px 10px 20px 20px 10px 20px 10px;
    background: lightgray;
    column-gap: 10px;
  }
  .subgrid {
    display: grid;
    grid-template-columns: subgrid;
    grid-template-rows: subgrid;
    background: teal;
    grid-column: 2 / -2;
    grid-row: 3 / -3;
    background: orange;
  }
  .item {
    background: purple;
  }
  .col-gap-group {
    position: absolute;
    top: 0px;
    left: 20px;
    display: flex;
    gap: 20px;
    height: 80px;
  }
  .col-gap {
    width: 10px;
    background: skyblue;
  }
  .row-gap-group {
    position: absolute;
    top: 20px;
    left: 0px;
    display: flex;
    flex-direction: column;
    row-gap: 20px;
    width: 140px;
  }
  .row-gap {
    background: cornflowerblue;
    height: 10px;
  }
```

```json
{
  "errors": 6,
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
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
